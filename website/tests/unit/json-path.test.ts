import assert from "node:assert/strict";
import { test } from "node:test";
import { json } from "@codemirror/lang-json";
import { EditorState } from "@codemirror/state";
import { rangeForPath } from "../../src/scripts/json-path.ts";

const DOC = '{ "subdivision": { "periods": [4, 5, 4] }, "week": "monday" }';
const state = EditorState.create({ doc: DOC, extensions: [json()] });
const text = (range: { from: number; to: number } | null) => (range ? DOC.slice(range.from, range.to) : null);

test("a path to a field finds its value", () => {
  assert.equal(text(rangeForPath(state, "week")), '"monday"');
  assert.equal(text(rangeForPath(state, "subdivision.periods")), "[4, 5, 4]");
  assert.equal(text(rangeForPath(state, "subdivision.periods[1]")), "5");
});

test("a missing field marks the deepest field that exists", () => {
  assert.equal(text(rangeForPath(state, "subdivision.extra_week")), '"subdivision"');
  assert.equal(text(rangeForPath(state, "subdivision.periods[9]")), '"periods"');
  assert.equal(text(rangeForPath(state, "week.starts_on")), '"week"');
});

test("a path that starts at a missing field, or names the whole input, finds nothing", () => {
  assert.equal(rangeForPath(state, "naming.default"), null);
  assert.equal(rangeForPath(state, ""), null);
});

test("an editor with no JSON value finds nothing", () => {
  const empty = EditorState.create({ doc: "", extensions: [json()] });
  assert.equal(rangeForPath(empty, "year"), null);
});
