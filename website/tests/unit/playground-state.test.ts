import assert from "node:assert/strict";
import { test } from "node:test";
import { decodeState, encodeState, type PlaygroundState } from "../../src/lib/playground-state.ts";

test("state round-trips through the fragment, including text that is not ASCII", () => {
  const state: PlaygroundState = {
    definition: '{\n  "naming": { "templates": { "year": "Exercice {yyyy} € · 会計年度 · ünï 😀" } }\n}',
    date: "2024-02-03",
    year: 2023,
  };
  const hash = encodeState(state);
  assert.match(hash, /^#s=[A-Za-z0-9_-]+$/, "the fragment is URL-safe base64 without padding");
  assert.deepEqual(decodeState(hash), state);
});

test("a definition that is not valid JSON is kept exactly as typed", () => {
  const state: PlaygroundState = { definition: '{"year": ', date: "not a date", year: -1 };
  assert.deepEqual(decodeState(encodeState(state)), state);
});

test("no fragment means no state, and an unreadable one throws", () => {
  assert.equal(decodeState(""), null);
  assert.equal(decodeState("#other"), null);
  assert.throws(() => decodeState("#s=not-a-state"));
  assert.throws(() => decodeState(encodeState({ definition: "", date: "", year: 1 }).replace(/.$/, "!")));
});

test("a fragment with the wrong shape throws", () => {
  const encode = (value: unknown) => "#s=" + Buffer.from(JSON.stringify(value)).toString("base64url");
  assert.throws(() => decodeState(encode(["{}", "2024-02-03"])), /unexpected shape/);
  assert.throws(() => decodeState(encode(["{}", "2024-02-03", 2023.5])), /unexpected shape/);
  assert.throws(() => decodeState(encode({ definition: "{}" })), /unexpected shape/);
});
