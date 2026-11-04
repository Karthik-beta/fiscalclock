/**
 * What the playground keeps in the URL fragment, so a link reproduces a
 * session. The date is always typed by the reader or taken from the example,
 * never from the clock (AGENTS.md design rule 6).
 */
export interface PlaygroundState {
  /** The definition exactly as typed, which may be invalid JSON. */
  definition: string;
  date: string;
  year: number;
}

const PREFIX = "#s=";

export function encodeState(state: PlaygroundState): string {
  const bytes = new TextEncoder().encode(JSON.stringify([state.definition, state.date, state.year]));
  let binary = "";
  for (const b of bytes) binary += String.fromCharCode(b);
  return PREFIX + btoa(binary).replaceAll("+", "-").replaceAll("/", "_").replace(/=+$/, "");
}

/** `null` when there is no state in the fragment; throws when there is one that cannot be read. */
export function decodeState(hash: string): PlaygroundState | null {
  if (!hash.startsWith(PREFIX)) return null;
  const base64 = hash.slice(PREFIX.length).replaceAll("-", "+").replaceAll("_", "/");
  const bytes = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
  const value: unknown = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
  if (
    !Array.isArray(value) ||
    value.length !== 3 ||
    typeof value[0] !== "string" ||
    typeof value[1] !== "string" ||
    !Number.isInteger(value[2])
  ) {
    throw new Error("unexpected shape");
  }
  return { definition: value[0], date: value[1], year: value[2] };
}
