"""Build fiscalclock's design tokens into CSS custom properties and resolved JSON.

The sources are W3C Design Tokens Community Group (DTCG) 2025.10 files in
design/tokens/, tied together by fiscalclock.resolver.json:

- Format module: https://www.designtokens.org/TR/2025.10/format/
- Color module: https://www.designtokens.org/TR/2025.10/color/
- Resolver module: https://www.designtokens.org/TR/2025.10/resolver/

This script implements the subset of the format that the token files use and
rejects everything else, so a token it cannot read is an error, never a guess.

Outputs, all in design/tokens/generated/:

- tokens.css: primitives on :root; light semantic tokens on :root; dark
  semantic tokens under prefers-color-scheme (unless data-theme="light") and
  under [data-theme="dark"], so an explicit choice always wins.
- tokens.light.json, tokens.dark.json: every token resolved to a CSS value,
  keyed by custom property name, for tools that take values in configuration.
- contrast.md: every pair in design/tokens/contrast-checks.json with its WCAG
  2.2 ratio in both themes. A pair below its minimum fails the build.

Usage, from the repository root:

    python -I design/scripts/build_tokens.py            # write the outputs
    python -I design/scripts/build_tokens.py --check    # fail if they are stale
    python -I design/scripts/build_tokens.py --self-test
"""

from __future__ import annotations

import argparse
import difflib
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from contrast import contrast_ratio, format_ratio  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
TOKENS = ROOT / "design" / "tokens"
BUILD = TOKENS / "generated"
RESOLVER = TOKENS / "fiscalclock.resolver.json"
CHECKS = TOKENS / "contrast-checks.json"
PREFIX = "fc"
DARK_SELECTOR = ':root[data-theme="dark"]'

NAME = re.compile(r"^[a-z0-9]+(-[a-z0-9]+)*$")
ALIAS = re.compile(r"^\{([a-z0-9-]+(?:\.[a-z0-9-]+)*)\}$")
HEX = re.compile(r"^#[0-9a-f]{6}$")
TOKEN_PROPS = {"$value", "$type", "$description"}
GROUP_PROPS = {"$type", "$description"}
TYPOGRAPHY_KEYS = ("fontFamily", "fontSize", "fontWeight", "letterSpacing", "lineHeight")
TYPOGRAPHY_TYPES = {"fontFamily": "fontFamily", "fontSize": "dimension", "fontWeight": "fontWeight",
                    "letterSpacing": "dimension", "lineHeight": "number"}


class TokenError(Exception):
    """A token file breaks the format or the project's rules."""


def _no_duplicates(pairs: list[tuple[str, object]]) -> dict:
    seen: dict = {}
    for key, value in pairs:
        if key in seen:
            raise TokenError(f"duplicate key {key!r}")
        seen[key] = value
    return seen


def load_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=_no_duplicates)
    except TokenError as err:
        raise TokenError(f"{path.name}: {err}") from None


def flatten(tree: dict, source: str, prefix: tuple[str, ...] = (), inherited: str | None = None,
            out: dict | None = None) -> dict:
    """Return {path: {"value", "type", "source"}} for every token in a DTCG tree."""
    out = {} if out is None else out
    group_type = tree.get("$type", inherited)
    for key, node in tree.items():
        if key.startswith("$"):
            allowed = TOKEN_PROPS if "$value" in tree else GROUP_PROPS
            if key not in allowed:
                raise TokenError(f"{source}: unsupported property {key} at {'.'.join(prefix) or '(root)'}")
            continue
        if not NAME.match(key):
            raise TokenError(f"{source}: name {key!r} at {'.'.join(prefix) or '(root)'} is not lowercase-hyphenated")
        path = prefix + (key,)
        if not isinstance(node, dict):
            raise TokenError(f"{source}: {'.'.join(path)} is neither a token nor a group")
        if "$value" in node:
            for prop in node:
                if prop not in TOKEN_PROPS:
                    raise TokenError(f"{source}: unsupported property {prop} on token {'.'.join(path)}")
            out[".".join(path)] = {"value": node["$value"], "type": node.get("$type", group_type), "source": source}
        else:
            flatten(node, source, path, group_type, out)
    return out


def check_value(path: str, ttype: str | None, value: object) -> None:
    """Validate a non-alias value against the shapes the format defines for its type."""
    def bad(why: str) -> TokenError:
        return TokenError(f"{path}: {why}")

    if ttype is None:
        raise bad("no $type on the token or any parent group")
    if ttype == "color":
        if not isinstance(value, dict) or set(value) - {"colorSpace", "components", "alpha", "hex"}:
            raise bad("color must be an object with colorSpace, components, optional alpha and hex")
        if value.get("colorSpace") != "srgb":
            raise bad("only the srgb colour space is used in this project")
        comps, hex_ = value.get("components"), value.get("hex")
        if not (isinstance(comps, list) and len(comps) == 3 and all(isinstance(c, (int, float)) and 0 <= c <= 1 for c in comps)):
            raise bad("srgb components must be three numbers in [0, 1]")
        if not (isinstance(hex_, str) and HEX.match(hex_)):
            raise bad("hex must be a lowercase 6-digit CSS hex colour")
        if [round(c * 255) for c in comps] != [int(hex_[i:i + 2], 16) for i in (1, 3, 5)]:
            raise bad(f"components {comps} do not match hex {hex_}")
        if value.get("alpha", 1) != 1:
            raise bad("alpha other than 1 is not used in this project")
    elif ttype == "dimension":
        if not (isinstance(value, dict) and set(value) == {"value", "unit"} and value["unit"] in ("px", "rem")
                and isinstance(value["value"], (int, float))):
            raise bad("dimension must be {value: number, unit: px or rem}")
    elif ttype == "duration":
        if not (isinstance(value, dict) and set(value) == {"value", "unit"} and value["unit"] in ("ms", "s")
                and isinstance(value["value"], (int, float))):
            raise bad("duration must be {value: number, unit: ms or s}")
    elif ttype == "fontFamily":
        if not (isinstance(value, str) or (isinstance(value, list) and value and all(isinstance(v, str) for v in value))):
            raise bad("fontFamily must be a string or a non-empty list of strings")
    elif ttype == "fontWeight":
        if not (isinstance(value, int) and 1 <= value <= 1000):
            raise bad("fontWeight must be a number from 1 to 1000")
    elif ttype == "number":
        if not isinstance(value, (int, float)) or isinstance(value, bool):
            raise bad("number must be a JSON number")
    elif ttype == "cubicBezier":
        if not (isinstance(value, list) and len(value) == 4 and all(isinstance(v, (int, float)) for v in value)
                and 0 <= value[0] <= 1 and 0 <= value[2] <= 1):
            raise bad("cubicBezier must be four numbers with x values in [0, 1]")
    elif ttype == "typography":
        if not (isinstance(value, dict) and set(value) == set(TYPOGRAPHY_KEYS)):
            raise bad(f"typography must have exactly {', '.join(TYPOGRAPHY_KEYS)}")
    else:
        raise bad(f"type {ttype} is not used in this project")


def resolve(tokens: dict) -> dict:
    """Return {path: (type, resolved value, alias target or None)}; fail on missing, circular or mistyped aliases."""
    done: dict = {}

    def follow(path: str, stack: list[str]) -> tuple[str, object]:
        if path in done:
            return done[path][0], done[path][1]
        if path in stack:
            chain = stack[stack.index(path):] + [path]
            raise TokenError("circular reference: " + " -> ".join(chain))
        if path not in tokens:
            raise TokenError(f"{stack[-1] if stack else path}: reference to missing token {{{path}}}")
        tok = tokens[path]
        value, ttype = tok["value"], tok["type"]
        match = ALIAS.match(value) if isinstance(value, str) else None
        if match:
            target_type, target_value = follow(match.group(1), stack + [path])
            if ttype is not None and ttype != target_type:
                raise TokenError(f"{path}: type {ttype} aliases {{{match.group(1)}}} of type {target_type}")
            done[path] = (target_type, target_value, match.group(1))
            return target_type, target_value
        check_value(path, ttype, value)
        if ttype == "typography":
            parts = {}
            for key in TYPOGRAPHY_KEYS:
                sub = value[key]
                sub_match = ALIAS.match(sub) if isinstance(sub, str) else None
                if sub_match:
                    sub_type, sub_value = follow(sub_match.group(1), stack + [path])
                    if sub_type != TYPOGRAPHY_TYPES[key]:
                        raise TokenError(f"{path}.{key}: expects {TYPOGRAPHY_TYPES[key]}, got {sub_type}")
                    parts[key] = (sub_value, sub_match.group(1))
                else:
                    check_value(f"{path}.{key}", TYPOGRAPHY_TYPES[key], sub)
                    parts[key] = (sub, None)
            value = parts
        done[path] = (ttype, value, None)
        return ttype, value

    for path in sorted(tokens):
        follow(path, [])
    return done


def load_contexts(resolver_path: Path) -> tuple[dict, dict[str, dict]]:
    """Read the resolver; return (primitives, {context: merged tokens})."""
    doc = load_json(resolver_path)
    if doc.get("version") != "2025.10":
        raise TokenError(f"{resolver_path.name}: version must be 2025.10")
    base = resolver_path.parent

    def read_sources(sources: list) -> dict:
        merged: dict = {}
        for src in sources:
            ref = src.get("$ref", "")
            if set(src) != {"$ref"} or "/" in ref or "\\" in ref or not ref.endswith(".tokens.json"):
                raise TokenError(f"{resolver_path.name}: unsupported source {src!r}")
            merged.update(flatten(load_json(base / ref), ref))
        return merged

    order = [step.get("$ref") for step in doc.get("resolutionOrder", [])]
    if order != ["#/sets/primitives", "#/modifiers/theme"]:
        raise TokenError(f"{resolver_path.name}: resolutionOrder must be the primitives set, then the theme modifier")
    primitives = read_sources(doc["sets"]["primitives"]["sources"])
    theme = doc["modifiers"]["theme"]
    if sorted(theme["contexts"]) != ["dark", "light"] or theme.get("default") != "light":
        raise TokenError(f"{resolver_path.name}: the theme modifier needs light and dark contexts, default light")
    contexts = {}
    for name, sources in theme["contexts"].items():
        semantic = read_sources(sources)
        clash = sorted(set(semantic) & set(primitives))
        if clash:
            raise TokenError(f"theme {name} redefines primitives: {', '.join(clash)}")
        contexts[name] = {**primitives, **semantic}
    light_only = sorted(set(contexts["light"]) - set(contexts["dark"]))
    dark_only = sorted(set(contexts["dark"]) - set(contexts["light"]))
    if light_only or dark_only:
        raise TokenError(f"themes differ: only in light {light_only}, only in dark {dark_only}")
    for path in contexts["light"]:
        if contexts["light"][path]["type"] != contexts["dark"][path]["type"]:
            raise TokenError(f"{path}: type differs between themes")
    return primitives, contexts


def var(path: str) -> str:
    return f"--{PREFIX}-" + path.replace(".", "-")


def css_value(ttype: str, value: object) -> str:
    if ttype == "color":
        return value["hex"]
    if ttype in ("dimension", "duration"):
        number = value["value"]
        return f"{number:g}{value['unit']}" if number else "0"
    if ttype == "fontFamily":
        names = [value] if isinstance(value, str) else value
        generic = {"serif", "sans-serif", "monospace", "system-ui", "ui-monospace", "-apple-system"}
        return ", ".join(n if n in generic or re.fullmatch(r"[A-Za-z-]+", n) and n[0].islower() else f'"{n}"' for n in names)
    if ttype in ("fontWeight", "number"):
        return f"{value:g}"
    if ttype == "cubicBezier":
        return "cubic-bezier(" + ", ".join(f"{v:g}" for v in value) + ")"
    raise TokenError(f"no CSS form for type {ttype}")


def declarations(paths: list[str], resolved: dict, use_aliases: bool) -> list[str]:
    """CSS declarations for the given token paths. Aliases stay visible as var() references."""
    lines = []
    for path in paths:
        ttype, value, target = resolved[path]
        if ttype == "typography":
            for key in TYPOGRAPHY_KEYS:
                sub_value, sub_target = value[key]
                prop = var(path) + "-" + re.sub(r"([A-Z])", r"-\1", key).lower()
                shown = f"var({var(sub_target)})" if sub_target and use_aliases else css_value(TYPOGRAPHY_TYPES[key], sub_value)
                lines.append(f"  {prop}: {shown};")
            continue
        shown = f"var({var(target)})" if target and use_aliases else css_value(ttype, value)
        lines.append(f"  {var(path)}: {shown};")
    return lines


def flat_values(resolved: dict) -> dict:
    out = {}
    for path in sorted(resolved):
        for line in declarations([path], resolved, use_aliases=False):
            name, value = line.strip().rstrip(";").split(": ", 1)
            out[name] = value
    return out


def build(resolver_path: Path = RESOLVER, checks_path: Path = CHECKS) -> dict[str, str]:
    """Return {file name: contents} for everything in design/tokens/generated/."""
    primitives, contexts = load_contexts(resolver_path)
    resolved = {name: resolve(tokens) for name, tokens in contexts.items()}
    prim_paths = sorted(primitives)
    sem_paths = sorted(set(contexts["light"]) - set(primitives))
    header = ["/* Generated by design/scripts/build_tokens.py from design/tokens/*.tokens.json. Do not edit. */", ""]
    light = declarations(sem_paths, resolved["light"], True)
    dark = declarations(sem_paths, resolved["dark"], True)
    css = header + [":root {", "  color-scheme: light;", *declarations(prim_paths, resolved["light"], True), *light, "}", "",
                    "@media (prefers-color-scheme: dark) {", '  :root:not([data-theme="light"]) {', "    color-scheme: dark;",
                    *("  " + line for line in dark), "  }", "}", "", DARK_SELECTOR + " {", "  color-scheme: dark;", *dark, "}", ""]
    files = {"tokens.css": "\n".join(css)}
    for name in ("light", "dark"):
        files[f"tokens.{name}.json"] = json.dumps(flat_values(resolved[name]), indent=2) + "\n"
    files["contrast.md"] = contrast_report(load_json(checks_path), resolved)
    return files


def contrast_report(checks: dict, resolved: dict) -> str:
    rows, failures = [], []
    for check in checks["pairs"]:
        fg, bg, minimum = check["foreground"], check["background"], check["minimum"]
        cells = []
        for theme in ("light", "dark"):
            for path in (fg, bg):
                if path not in resolved[theme] or resolved[theme][path][0] != "color":
                    raise TokenError(f"contrast check names {path}, which is not a colour token")
            fg_hex, bg_hex = resolved[theme][fg][1]["hex"], resolved[theme][bg][1]["hex"]
            ratio = contrast_ratio(fg_hex, bg_hex)
            if ratio < minimum:
                failures.append(f"{theme}: {fg} on {bg} is {format_ratio(ratio)}, below {minimum}:1")
            cells.append(f"`{fg_hex}` on `{bg_hex}`: {format_ratio(ratio)}")
        rows.append(f"| `{fg}` on `{bg}` | {check['criterion']} | {minimum}:1 | {cells[0]} | {cells[1]} |")
    if failures:
        raise TokenError("contrast failures:\n  " + "\n  ".join(failures))
    return "\n".join([
        "<!-- Generated by design/scripts/build_tokens.py from design/tokens/contrast-checks.json. Do not edit. -->",
        "<!-- cspell:words WCAG -->",
        "",
        "# Contrast report",
        "",
        "Every pair below is checked in both themes on each build; a ratio below its minimum fails the build. "
        "Ratios follow the WCAG 2.2 definition and are truncated to two decimals, never rounded up.",
        "",
        "| Pair | WCAG 2.2 criterion | Minimum | Light | Dark |",
        "| --- | --- | --- | --- | --- |",
        *rows, ""])


def self_test() -> int:
    """Each fixture must be rejected with the expected message."""
    import tempfile

    good = {"palette": {"$type": "color", "a": {"$value": {"colorSpace": "srgb", "components": [0, 0, 0], "hex": "#000000"}}}}
    cases = {
        "circular reference": {"color": {"$type": "color", "x": {"$value": "{color.y}"}, "y": {"$value": "{color.x}"}}},
        "reference to missing token": {"color": {"$type": "color", "x": {"$value": "{palette.nope}"}}},
        "aliases": {"color": {"x": {"$type": "dimension", "$value": "{palette.a}"}}},
        "themes differ": None,
        "do not match hex": {"color": {"$type": "color", "x": {"$value": {"colorSpace": "srgb", "components": [1, 1, 1], "hex": "#000000"}}}},
    }
    failed = 0
    for expected, semantic in cases.items():
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            (base / "p.tokens.json").write_text(json.dumps(good), encoding="utf-8", newline="\n")
            light = semantic or {"color": {"$type": "color", "x": {"$value": "{palette.a}"}}}
            dark = semantic or {"color": {"$type": "color", "y": {"$value": "{palette.a}"}}}
            (base / "l.tokens.json").write_text(json.dumps(light), encoding="utf-8", newline="\n")
            (base / "d.tokens.json").write_text(json.dumps(dark), encoding="utf-8", newline="\n")
            (base / "r.json").write_text(json.dumps({
                "version": "2025.10", "sets": {"primitives": {"sources": [{"$ref": "p.tokens.json"}]}},
                "modifiers": {"theme": {"contexts": {"light": [{"$ref": "l.tokens.json"}], "dark": [{"$ref": "d.tokens.json"}]}, "default": "light"}},
                "resolutionOrder": [{"$ref": "#/sets/primitives"}, {"$ref": "#/modifiers/theme"}]}), encoding="utf-8", newline="\n")
            (base / "c.json").write_text(json.dumps({"pairs": []}), encoding="utf-8", newline="\n")
            try:
                build(base / "r.json", base / "c.json")
                print(f"FAIL {expected}: accepted")
                failed += 1
            except TokenError as err:
                ok = expected in str(err)
                failed += not ok
                print(f"{'ok  ' if ok else 'FAIL'} {expected}: {err}".splitlines()[0])
    print(f"{len(cases) - failed}/{len(cases)} invalid inputs rejected as expected")
    mixed = ["IBM Plex Sans", "system-ui", "-apple-system", "Segoe UI", "sans-serif"]
    want = '"IBM Plex Sans", system-ui, -apple-system, "Segoe UI", sans-serif'
    got = css_value("fontFamily", mixed)
    print(f"{'ok  ' if got == want else 'FAIL'} fontFamily quotes names and leaves generic keywords bare: {got}")
    failed += got != want
    return 1 if failed else 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--check", action="store_true", help="fail if the committed outputs are out of date")
    parser.add_argument("--self-test", action="store_true", help="check that invalid token files are rejected")
    args = parser.parse_args(argv)
    if args.self_test:
        return self_test()
    try:
        files = build()
    except TokenError as err:
        print(f"error: {err}", file=sys.stderr)
        return 1
    stale = 0
    for name, text in files.items():
        target = BUILD / name
        if args.check:
            current = target.read_text(encoding="utf-8") if target.exists() else ""
            if current != text:
                stale += 1
                sys.stdout.writelines(difflib.unified_diff(current.splitlines(True), text.splitlines(True), f"committed/{name}", f"generated/{name}"))
        else:
            BUILD.mkdir(exist_ok=True)
            target.write_text(text, encoding="utf-8", newline="\n")
            print(f"wrote {target.relative_to(ROOT).as_posix()}")
    if args.check:
        print("design tokens are up to date" if not stale else f"{stale} generated file(s) are out of date; run without --check")
    return 1 if stale else 0


if __name__ == "__main__":
    raise SystemExit(main())
