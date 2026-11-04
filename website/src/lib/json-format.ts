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

/** Text, or a place between two members of an object nested `depth` levels below a block where a line may break. */
type Piece = string | { depth: number };

/**
 * Formats a definition compactly: each top-level block starts a line, and a
 * block longer than `width` columns breaks only between object members, at the
 * shallowest nesting level that keeps the line within `width`. A continued line
 * is indented by its nesting level. The landing page shows a definition this way.
 */
export function formatJsonBlocks(value: Record<string, unknown>, width: number): string {
  const entries = Object.entries(value);
  const blocks = entries.map(([k, v], i) => fill([`  ${JSON.stringify(k)}: `, ...pieces(v, 1), i < entries.length - 1 ? "," : ""], width));
  return `{\n${blocks.join("\n")}\n}`;
}

function pieces(value: unknown, depth: number): Piece[] {
  if (value === null || typeof value !== "object" || Array.isArray(value) || Object.keys(value).length === 0) return [oneLine(value)];
  const out: Piece[] = ["{ "];
  Object.entries(value).forEach(([k, v], i) => {
    if (i > 0) out.push(",", { depth });
    out.push(`${JSON.stringify(k)}: `, ...pieces(v, depth + 1));
  });
  out.push(" }");
  return out;
}

function fill(all: Piece[], width: number): string {
  // runs[i] is the text between break i - 1 and break i; depths[i] is the depth of break i.
  const runs = [""];
  const depths: number[] = [];
  for (const p of all) {
    if (typeof p === "string") {
      runs[runs.length - 1] += p;
    } else {
      depths.push(p.depth);
      runs.push("");
    }
  }
  const lines: string[] = [];
  let start = 0;
  let indent = "";
  while (true) {
    // Each break follows a comma, so runs on one line are joined by a space.
    const text = (end: number) => indent + runs.slice(start, end + 1).join(" ");
    if (text(runs.length - 1).length <= width || start === runs.length - 1) {
      lines.push(text(runs.length - 1));
      return lines.join("\n");
    }
    const fitting = depths.map((_, b) => b).filter((b) => b >= start && text(b).length <= width);
    const shallowest = Math.min(...fitting.map((b) => depths[b]!));
    const end = fitting.length === 0 ? start : fitting.filter((b) => depths[b] === shallowest).at(-1)!;
    lines.push(text(end));
    indent = "  ".repeat(depths[end]! + 1);
    start = end + 1;
  }
}

function oneLine(value: unknown): string {
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(oneLine).join(", ")}]`;
  const entries = Object.entries(value).map(([k, v]) => `${JSON.stringify(k)}: ${oneLine(v)}`);
  return entries.length === 0 ? "{}" : `{ ${entries.join(", ")} }`;
}
