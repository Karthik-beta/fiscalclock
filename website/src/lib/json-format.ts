/**
 * Formats JSON for people: a value stays on one line when it fits in `width`
 * columns, otherwise its members go on separate lines. Arrays of numbers such
 * as `periods` then read as one row, the way the README writes definitions.
 */
export function formatJson(value: unknown, width = 72, indent = "", lead = 0): string {
  const inline = oneLine(value);
  if (indent.length + lead + inline.length <= width || value === null || typeof value !== "object") return inline;
  const inner = indent + "  ";
  if (Array.isArray(value)) {
    return `[\n${value.map((v) => inner + formatJson(v, width, inner)).join(",\n")}\n${indent}]`;
  }
  const entries = Object.entries(value).map(([k, v]) => {
    const key = `${JSON.stringify(k)}: `;
    return inner + key + formatJson(v, width, inner, key.length);
  });
  return `{\n${entries.join(",\n")}\n${indent}}`;
}

function oneLine(value: unknown): string {
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(oneLine).join(", ")}]`;
  const entries = Object.entries(value).map(([k, v]) => `${JSON.stringify(k)}: ${oneLine(v)}`);
  return entries.length === 0 ? "{}" : `{ ${entries.join(", ")} }`;
}
