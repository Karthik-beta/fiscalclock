/**
 * Finds the text an error's JSON path points at, using the JSON syntax tree
 * CodeMirror already has. Paths are the core's dotted form with [i] for
 * array indexes (spec/api-v1.md, "Error"), such as `subdivision.periods[3]`.
 */
import { syntaxTree } from "@codemirror/language";
import type { EditorState } from "@codemirror/state";

type Node = ReturnType<typeof syntaxTree>["topNode"];
export interface Range {
  from: number;
  to: number;
}

const VALUES = new Set(["Object", "Array", "String", "Number", "True", "False", "Null"]);

function segments(path: string): (string | number)[] {
  return [...path.matchAll(/([^.[\]]+)|\[(\d+)\]/g)].map((m) => (m[2] === undefined ? m[1]! : Number(m[2])));
}

function children(node: Node): Node[] {
  const out: Node[] = [];
  for (let child = node.firstChild; child; child = child.nextSibling) out.push(child);
  return out;
}

/**
 * The value at `path`. When the path names a field that does not exist (such
 * as a missing `subdivision.extra_week`), the name of the deepest field that
 * does exist. `null` for the whole input or when nothing on the path exists.
 */
export function rangeForPath(state: EditorState, path: string): Range | null {
  let node = children(syntaxTree(state).topNode).find((n) => VALUES.has(n.name));
  let found: Range | null = null;
  for (const seg of segments(path)) {
    if (!node) return found;
    let next: Node | undefined;
    if (typeof seg === "string" && node.name === "Object") {
      for (const prop of children(node).filter((n) => n.name === "Property")) {
        const name = prop.getChild("PropertyName");
        if (!name || readKey(state, name) !== seg) continue;
        found = { from: name.from, to: name.to };
        next = children(prop).find((n) => VALUES.has(n.name));
        break;
      }
    } else if (typeof seg === "number" && node.name === "Array") {
      next = children(node).filter((n) => VALUES.has(n.name))[seg];
      if (next) found = { from: next.from, to: next.to };
    }
    if (!next) return found;
    node = next;
  }
  return node && found ? { from: node.from, to: node.to } : found;
}

function readKey(state: EditorState, node: Node): string | null {
  try {
    return JSON.parse(state.sliceDoc(node.from, node.to));
  } catch {
    return null;
  }
}
