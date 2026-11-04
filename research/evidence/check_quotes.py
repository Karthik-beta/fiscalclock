"""Check that every quote in claims.json appears in its source page.

Standard library only. Run from the repository root:

    python -I research/evidence/check_quotes.py            # fetch every source
    python -I research/evidence/check_quotes.py --offline  # validate claims.json only

Each claim with a `quote` and an http(s) `fetch_url` or `source_url` is
reported as FOUND, NOT FOUND or FETCH FAILED. sec.gov is never requested and
is always reported as FETCH FAILED. The page text is compared after removing
tags, decoding entities, applying Unicode NFKC, mapping curly quotes to
straight quotes and collapsing whitespace. The normalisation is used for
comparison only; the register keeps the quote as the source prints it.

Exit status is 1 when any quote is NOT FOUND or claims.json is malformed.
FETCH FAILED (a network error, or a host that refuses automated clients) is
reported but does not fail the run, because it says nothing about the quote.
"""

from __future__ import annotations

import argparse
import datetime
import gzip
import json
import re
import sys
import unicodedata
import urllib.error
import urllib.parse
import urllib.request
import zlib
from html.parser import HTMLParser
from pathlib import Path

USER_AGENT = "fiscalclock-evidence-check (+https://github.com/Karthik-beta/fiscalclock)"
HERE = Path(__file__).resolve().parent
CLAIMS = HERE / "claims.json"
CHECKS_README = HERE.parent / "verification" / "README.md"
STATUSES = {"verified", "quote corrected", "fetch failed", "unverified"}
METHODS = {"fetched page", "script check", "filing", "secondary"}
LAYOUTS = {"claims", "checks", "open"}
REQUIRED = ("id", "topic", "claim", "source_label", "source_url", "quote", "evidence",
            "check", "method", "status", "verified_on")
OPTIONAL = ("fetch_url", "method_note", "status_note", "appears_in")
# sec.gov answers 403 unless the User-Agent carries a contact email address, which
# this project does not send, so its pages are never requested.
NOT_FETCHED = {"sec.gov": "not fetched: sec.gov requires a contact email in the User-Agent"}

BLOCK_MARKERS = (
    "Undeclared Automated Tool",
    "Request Rate Threshold Exceeded",
    "Just a moment...",
    "Attention Required! | Cloudflare",
    "Access Denied",
    "Enable JavaScript and cookies to continue",
    "Please verify you are a human",
    "Client Challenge",
)

QUOTE_MAP = str.maketrans({
    "‘": "'", "’": "'", "‚": "'", "‛": "'", "′": "'",
    "“": '"', "”": '"', "„": '"', "‟": '"', "″": '"',
    "­": None, "​": None, "‌": None, "‍": None, "﻿": None,
})


class TextExtractor(HTMLParser):
    SKIP = {"script", "style", "noscript", "template", "svg"}
    BLOCK = {"p", "div", "br", "li", "tr", "td", "th", "h1", "h2", "h3", "h4", "h5", "h6",
             "section", "article", "table", "ul", "ol", "dd", "dt", "pre", "blockquote"}

    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.parts: list[str] = []
        self.skipping = 0

    def handle_starttag(self, tag, attrs):
        if tag in self.SKIP:
            self.skipping += 1
        elif tag in self.BLOCK:
            self.parts.append(" ")

    def handle_endtag(self, tag):
        if tag in self.SKIP and self.skipping:
            self.skipping -= 1
        elif tag in self.BLOCK:
            self.parts.append(" ")

    def handle_data(self, data):
        if not self.skipping:
            self.parts.append(data)


def normalise(text: str) -> str:
    text = unicodedata.normalize("NFKC", text).translate(QUOTE_MAP)
    return re.sub(r"\s+", " ", text).strip()


def json_strings(value) -> list[str]:
    if isinstance(value, str):
        return [value]
    if isinstance(value, dict):
        return [s for v in value.values() for s in json_strings(v)]
    if isinstance(value, list):
        return [s for v in value for s in json_strings(v)]
    return []


def page_text(body: str, content_type: str) -> str:
    if "json" in content_type:
        try:
            return "\n".join(json_strings(json.loads(body)))
        except ValueError:
            pass
    if "html" in content_type or body.lstrip()[:1] == "<":
        parser = TextExtractor()
        parser.feed(body)
        parser.close()
        return "".join(parser.parts)
    return body


def fetch(url: str) -> tuple[str | None, str]:
    """Return (text, problem). text is None when the fetch failed."""
    request = urllib.request.Request(url, headers={
        "User-Agent": USER_AGENT,
        "Accept": "text/html,application/xhtml+xml,application/json;q=0.9,*/*;q=0.8",
        "Accept-Encoding": "gzip",
        "Accept-Language": "en",
    })
    try:
        with urllib.request.urlopen(request, timeout=45) as response:
            raw = response.read()
            encoding = response.headers.get("Content-Encoding", "").lower()
            content_type = response.headers.get("Content-Type", "")
            charset = response.headers.get_content_charset() or "utf-8"
    except urllib.error.HTTPError as error:
        return None, f"HTTP {error.code}"
    except (urllib.error.URLError, TimeoutError, OSError) as error:
        return None, f"{type(error).__name__}: {getattr(error, 'reason', error)}"
    if encoding not in ("", "identity", "gzip"):
        return None, f"unsupported Content-Encoding {encoding!r}"
    try:
        if encoding == "gzip":
            raw = gzip.decompress(raw)
        body = raw.decode(charset, errors="replace")
    except (OSError, EOFError, zlib.error, LookupError) as error:
        return None, f"{type(error).__name__}: {error}"
    return page_text(body, content_type.lower()), ""


def quotes_of(claim: dict) -> list[str]:
    quote = claim.get("quote")
    if quote is None:
        return []
    return [quote] if isinstance(quote, str) else list(quote)


def is_iso_date(value) -> bool:
    try:
        datetime.date.fromisoformat(value)
    except (TypeError, ValueError):
        return False
    return True


def check_ids() -> set[str]:
    text = CHECKS_README.read_text(encoding="utf-8")
    return set(re.findall(r"^\| (C\d{2}a?) \|", text, flags=re.MULTILINE))


def validate(register) -> list[str]:
    if not isinstance(register, dict):
        return ["claims.json must be an object with last_reviewed, cspell_ignore, topics and claims"]
    problems = []
    last_reviewed = register.get("last_reviewed")
    if not is_iso_date(last_reviewed):
        problems.append(f"last_reviewed {last_reviewed!r} is not an ISO date")
    words = register.get("cspell_ignore")
    if not isinstance(words, list) or not all(isinstance(w, str) and w for w in words):
        problems.append("cspell_ignore must be a list of words")

    layouts = {}
    for topic in register.get("topics") or []:
        tid = topic.get("id")
        if tid in layouts:
            problems.append(f"topic {tid}: duplicate id")
        if topic.get("layout") not in LAYOUTS:
            problems.append(f"topic {tid}: layout {topic.get('layout')!r}")
        if not isinstance(topic.get("heading"), str):
            problems.append(f"topic {tid}: missing heading")
        layouts[tid] = topic.get("layout")
    if not layouts:
        problems.append("topics must be a non-empty list")

    claims = register.get("claims")
    if not isinstance(claims, list) or not claims:
        return problems + ["claims must be a non-empty list"]
    known_checks = check_ids()
    seen = set()
    for index, claim in enumerate(claims):
        if not isinstance(claim, dict):
            problems.append(f"#{index}: not an object")
            continue
        where = claim.get("id", f"#{index}")
        missing = [f for f in REQUIRED if f not in claim]
        unknown = [f for f in claim if f not in REQUIRED and f not in OPTIONAL]
        if missing:
            problems.append(f"{where}: missing {', '.join(missing)}")
        if unknown:
            problems.append(f"{where}: unknown field {', '.join(unknown)}")
        if not re.fullmatch(r"E\d{3}", str(claim.get("id"))):
            problems.append(f"{where}: id must look like E001")
        if claim.get("id") in seen:
            problems.append(f"{where}: duplicate id")
        seen.add(claim.get("id"))
        layout = layouts.get(claim.get("topic"))
        if layout is None:
            problems.append(f"{where}: unknown topic {claim.get('topic')!r}")
        if claim.get("status") not in STATUSES:
            problems.append(f"{where}: status {claim.get('status')!r}")
        url = claim.get("source_url")
        if claim.get("method") not in METHODS and not (claim.get("method") is None and url is None):
            problems.append(f"{where}: method {claim.get('method')!r} (null only without a source)")
        if url is not None and not claim.get("source_label"):
            problems.append(f"{where}: source_url without source_label")
        if (claim.get("method") == "script check") != (claim.get("check") is not None):
            problems.append(f"{where}: a check ID goes with method 'script check' and only with it")
        if claim.get("check") is not None and claim.get("check") not in known_checks:
            problems.append(f"{where}: check {claim.get('check')!r} is not in {CHECKS_README.name}")
        if (layout == "checks") != (claim.get("check") is not None):
            problems.append(f"{where}: claims in a checks topic need a check ID, others none")
        if (layout == "open") != bool(claim.get("appears_in")):
            problems.append(f"{where}: appears_in is required in an open topic and only there")
        quote = claim.get("quote")
        if not (quote is None or (isinstance(quote, str) and quote)
                or (isinstance(quote, list) and quote
                    and all(isinstance(q, str) and q for q in quote))):
            problems.append(f"{where}: quote must be null, a string or a list of strings")
        else:
            evidence = normalise(str(claim.get("evidence", "")))
            for q in quotes_of(claim):
                if normalise(q) not in evidence:
                    problems.append(f"{where}: quote not shown in evidence: {q!r}")
        if not is_iso_date(claim.get("verified_on")):
            problems.append(f"{where}: verified_on {claim.get('verified_on')!r} is not an ISO date")
        elif is_iso_date(last_reviewed) and claim["verified_on"] > last_reviewed:
            problems.append(f"{where}: verified_on is after last_reviewed")
    registered = {c.get("check") for c in claims if isinstance(c, dict)}
    for check in sorted(known_checks - registered):
        problems.append(f"check {check} from {CHECKS_README.name} has no claim")
    return problems


def not_fetched_reason(url: str) -> str:
    host = urllib.parse.urlsplit(url).hostname or ""
    for domain, reason in NOT_FETCHED.items():
        if host == domain or host.endswith("." + domain):
            return reason
    return ""


def fetch_target(claim: dict) -> str:
    return claim.get("fetch_url") or claim.get("source_url") or ""


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--offline", action="store_true", help="validate claims.json without fetching")
    parser.add_argument("--claims", type=Path, default=CLAIMS)
    parser.add_argument("--id", action="append", help="check only these claim ids")
    args = parser.parse_args()
    sys.stdout.reconfigure(encoding="utf-8")

    register = json.loads(args.claims.read_text(encoding="utf-8"))
    problems = validate(register)
    for problem in problems:
        print(f"[INVALID] {problem}")
    claims = register.get("claims", []) if isinstance(register, dict) else []

    quoted = [c for c in claims if isinstance(c, dict) and quotes_of(c)
              and fetch_target(c).startswith(("http://", "https://"))]
    if args.id:
        quoted = [c for c in quoted if c["id"] in args.id]

    if args.offline:
        print(f"{len(claims)} claims, {len(quoted)} with quotes; fetching skipped (--offline)")
        return 1 if problems else 0

    pages: dict[str, tuple[str | None, str]] = {}
    counts = {"FOUND": 0, "NOT FOUND": 0, "FETCH FAILED": 0}
    for claim in quoted:
        url = fetch_target(claim)
        if url not in pages:
            reason = not_fetched_reason(url)
            if reason:
                pages[url] = (None, reason)
            else:
                text, problem = fetch(url)
                pages[url] = (normalise(text) if text is not None else None, problem)
        text, problem = pages[url]
        missing = []
        if text is None:
            result = "FETCH FAILED"
        else:
            missing = [q for q in quotes_of(claim) if normalise(q) not in text]
            if not missing:
                result = "FOUND"
            elif any(marker in text for marker in BLOCK_MARKERS):
                result, problem, missing = "FETCH FAILED", "host refused the automated client", []
            else:
                result = "NOT FOUND"
        counts[result] += 1
        suffix = f" ({problem})" if problem else ""
        print(f"[{result}] {claim['id']} {url}{suffix}")
        for q in missing:
            print(f"    missing: {q}")

    print(f"{counts['FOUND']} found, {counts['NOT FOUND']} not found, "
          f"{counts['FETCH FAILED']} fetch failed, {len(pages)} pages")
    return 1 if counts["NOT FOUND"] or problems else 0


if __name__ == "__main__":
    sys.exit(main())
