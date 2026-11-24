"""Creates the issues in seed-issues.md, adds them to project 6 and sets their fields.

Usage: python -I seed_issues.py [--apply]. Without --apply it only validates and
prints the plan. Issues whose exact title already exists are skipped, so a rerun
continues where a failed run stopped.
"""

import json
import re
import subprocess
import sys
from pathlib import Path

REPO, OWNER, PROJECT = "Karthik-beta/fiscalclock", "Karthik-beta", 6
SOURCE = Path(__file__).with_name("seed-issues.md")


def gh(*args, input=None):
    r = subprocess.run(["gh", *args], capture_output=True, text=True, encoding="utf-8", input=input)
    if r.returncode:
        raise SystemExit(f"gh {' '.join(args[:3])}: {r.stderr}")
    return json.loads(r.stdout) if r.stdout.strip() else None


def parse():
    text = SOURCE.read_text(encoding="utf-8").split("\n## ", 1)[1]
    issues = []
    for chunk in ("## " + text).split("\n## ")[0:]:
        chunk = chunk.removeprefix("## ")
        title, rest = chunk.split("\n", 1)
        meta, body = {}, []
        for line in rest.strip("\n").splitlines():
            m = re.match(r"^- (labels|milestone|state|fields|parent): (.*)$", line)
            if m and not body:
                meta[m.group(1)] = m.group(2)
            else:
                body.append(line)
        fields = dict(kv.split("=", 1) for kv in meta["fields"].split("; "))
        issues.append({
            "title": title.strip(),
            "labels": [x.strip() for x in meta["labels"].split(",")],
            "milestone": meta["milestone"],
            "state": meta["state"],
            "parent": meta.get("parent"),
            "fields": fields,
            "body": "\n".join(body).strip() + "\n",
        })
    return issues


def main(apply):
    issues = parse()
    labels = {l["name"] for l in gh("label", "list", "-R", REPO, "--limit", "200", "--json", "name")}
    milestones = {m["title"]: m["number"] for m in gh("api", f"repos/{REPO}/milestones?state=all&per_page=100")}
    proj = gh("api", "graphql", "-f", "query=query($o:String!,$n:Int!){user(login:$o){projectV2(number:$n){id fields(first:40){nodes{... on ProjectV2SingleSelectField{id name options{id name}}}}}}}",
              "-f", f"o={OWNER}", "-F", f"n={PROJECT}")["data"]["user"]["projectV2"]
    options = {f["name"]: (f["id"], {o["name"]: o["id"] for o in f["options"]}) for f in proj["fields"]["nodes"] if f}
    titles = {i["title"] for i in issues}
    problems = []
    for i in issues:
        problems += [f"{i['title']}: unknown label {l!r}" for l in i["labels"] if l not in labels]
        if i["milestone"] not in milestones:
            problems.append(f"{i['title']}: unknown milestone {i['milestone']!r}")
        for k, v in i["fields"].items():
            if k not in options or v not in options[k][1]:
                problems.append(f"{i['title']}: unknown field value {k}={v}")
        if i["parent"] and i["parent"] not in titles:
            problems.append(f"{i['title']}: unknown parent {i['parent']!r}")
    if problems:
        raise SystemExit("\n".join(problems))
    print(f"{len(issues)} issues valid: {sum(i['state'] == 'open' for i in issues)} open, {sum(i['state'] == 'closed' for i in issues)} closed")
    if not apply:
        return

    existing = {x["title"]: x for x in gh("issue", "list", "-R", REPO, "--state", "all", "--limit", "500", "--json", "title,number,id")}
    created = {}
    for i in issues:
        if i["title"] in existing:
            created[i["title"]] = existing[i["title"]]
            print("exists", i["title"])
            continue
        payload = json.dumps({"title": i["title"], "body": i["body"], "labels": i["labels"], "milestone": milestones[i["milestone"]]})
        issue = gh("api", f"repos/{REPO}/issues", "--input", "-", input=payload)
        created[i["title"]] = {"number": issue["number"], "id": issue["node_id"], "rest_id": issue["id"]}
        if i["state"] == "closed":
            gh("api", "-X", "PATCH", f"repos/{REPO}/issues/{issue['number']}", "-f", "state=closed", "-f", "state_reason=completed")
        item = gh("api", "graphql", "-f", "query=mutation($p:ID!,$c:ID!){addProjectV2ItemById(input:{projectId:$p,contentId:$c}){item{id}}}",
                  "-f", f"p={proj['id']}", "-f", f"c={issue['node_id']}")["data"]["addProjectV2ItemById"]["item"]["id"]
        for k, v in i["fields"].items():
            fid, opts = options[k]
            gh("api", "graphql", "-f", "query=mutation($p:ID!,$i:ID!,$f:ID!,$o:String!){updateProjectV2ItemFieldValue(input:{projectId:$p,itemId:$i,fieldId:$f,value:{singleSelectOptionId:$o}}){projectV2Item{id}}}",
               "-f", f"p={proj['id']}", "-f", f"i={item}", "-f", f"f={fid}", "-f", f"o={opts[v]}")
        print(f"#{issue['number']} {i['state']:<6} {i['title']}")
    for i in issues:
        if i["parent"]:
            parent = created[i["parent"]]["number"]
            child = created[i["title"]]
            rest_id = child.get("rest_id") or gh("api", f"repos/{REPO}/issues/{child['number']}")["id"]
            subprocess.run(["gh", "api", f"repos/{REPO}/issues/{parent}/sub_issues", "-F", f"sub_issue_id={rest_id}"], capture_output=True)
    print("sub-issues linked")


if __name__ == "__main__":
    main("--apply" in sys.argv)
