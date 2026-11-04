"""A minimal JSON Schema validator for the keywords that vector.schema.json uses.

Supported: type, enum, required, properties, additionalProperties (false), items, minItems,
minLength, pattern, oneOf, and local $ref. Any other keyword in the schema is an error, so the
schema cannot silently grow past what this checks.
"""

from __future__ import annotations

import re

KNOWN = {"$schema", "$id", "$defs", "title", "description", "type", "enum", "required", "properties", "additionalProperties", "items", "minItems", "minLength", "pattern", "oneOf", "$ref"}
TYPES = {
    "object": lambda v: isinstance(v, dict),
    "array": lambda v: isinstance(v, list),
    "string": lambda v: isinstance(v, str),
    "null": lambda v: v is None,
    "boolean": lambda v: isinstance(v, bool),
    "integer": lambda v: isinstance(v, int) and not isinstance(v, bool),
    "number": lambda v: isinstance(v, (int, float)) and not isinstance(v, bool),
}


def errors(value, schema: dict, root: dict, path: str = "$") -> list[str]:
    unknown = set(schema) - KNOWN
    if unknown:
        return [f"{path}: schema uses unsupported keywords {sorted(unknown)}"]
    if "$ref" in schema:
        node = root
        for part in schema["$ref"].lstrip("#/").split("/"):
            node = node[part]
        return errors(value, node, root, path)
    out = []
    if "type" in schema:
        types = schema["type"] if isinstance(schema["type"], list) else [schema["type"]]
        if not any(TYPES[t](value) for t in types):
            return [f"{path}: expected {types}"]
    if "enum" in schema and value not in schema["enum"]:
        out.append(f"{path}: {value!r} not in enum")
    if isinstance(value, str):
        if len(value) < schema.get("minLength", 0):
            out.append(f"{path}: shorter than minLength")
        if "pattern" in schema and not re.search(schema["pattern"], value):
            out.append(f"{path}: does not match {schema['pattern']}")
    if isinstance(value, list):
        if len(value) < schema.get("minItems", 0):
            out.append(f"{path}: fewer than minItems")
        if "items" in schema:
            for i, item in enumerate(value):
                out += errors(item, schema["items"], root, f"{path}[{i}]")
    if isinstance(value, dict):
        for key in schema.get("required", []):
            if key not in value:
                out.append(f"{path}: missing {key}")
        props = schema.get("properties", {})
        for key, sub in value.items():
            if key in props:
                out += errors(sub, props[key], root, f"{path}.{key}")
            elif schema.get("additionalProperties") is False:
                out.append(f"{path}: unexpected key {key}")
    if "oneOf" in schema:
        passing = sum(1 for sub in schema["oneOf"] if not errors(value, sub, root, path))
        if passing != 1:
            out.append(f"{path}: matches {passing} oneOf branches, expected 1")
    return out
