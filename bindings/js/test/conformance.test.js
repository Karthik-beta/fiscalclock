// Runs every shared conformance vector through the npm package and writes the
// raw output of every case to target/transcripts/js.txt. The Rust and Python
// runners write the same transcript, and CI requires the three files to be
// identical byte for byte.

import assert from "node:assert/strict";
import { mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

import { callJson, validateJson } from "../dist/node.js";

const root = new URL("../../../", import.meta.url);
const vectors = new URL("conformance/vectors/", root);
const transcriptDir = new URL("target/transcripts/", root);

function vectorFiles(dir, prefix = "") {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) =>
    e.isDirectory() ? vectorFiles(new URL(`${e.name}/`, dir), `${prefix}${e.name}/`) : e.name.endsWith(".json") ? [`${prefix}${e.name}`] : [],
  );
}

function kind(v) {
  return v === null ? "null" : Array.isArray(v) ? "array" : typeof v;
}

function same(a, b) {
  if (kind(a) !== kind(b)) return false;
  if (kind(a) === "array") return a.length === b.length && a.every((x, i) => same(x, b[i]));
  if (kind(a) === "object") {
    const ka = Object.keys(a).sort();
    const kb = Object.keys(b).sort();
    return same(ka, kb) && ka.every((k) => same(a[k], b[k]));
  }
  return a === b;
}

function subset(want, got) {
  if (kind(want) === "object" && kind(got) === "object") return Object.entries(want).every(([k, v]) => k in got && subset(v, got[k]));
  return same(want, got);
}

function errorMatches(want, got) {
  return same(want.code, got.code) && same(want.path, got.path) && (!("details" in want) || same(want.details, got.details));
}

function runCase(definition, c) {
  if (c.op === "validate") return validateJson(definition);
  return callJson(definition, c.op, JSON.stringify(c.args));
}

function checkCase(out, c) {
  const got = JSON.parse(out);
  if ("expect" in c) return same(c.expect, got);
  if ("expect_subset" in c) return subset(c.expect_subset, got);
  const errors = kind(got) === "object" && Array.isArray(got.errors) ? got.errors : [];
  const want = c.expect_error;
  if (Array.isArray(want)) return want.length === errors.length && want.every((w, i) => errorMatches(w, errors[i]));
  return errors.length > 0 && errorMatches(want, errors[0]);
}

test("every conformance vector passes", () => {
  const files = vectorFiles(vectors).sort();
  assert.ok(files.length > 0, `no vectors under ${fileURLToPath(vectors)}`);
  const lines = [];
  const failures = [];
  for (const rel of files) {
    const vector = JSON.parse(readFileSync(new URL(rel, vectors), "utf8"));
    const definition = JSON.stringify(vector.definition);
    vector.cases.forEach((c, i) => {
      const out = runCase(definition, c);
      lines.push(`${rel}#${i}\t${out}\n`);
      if (!checkCase(out, c)) failures.push(`${rel}#${i}: ${c.op} ${JSON.stringify(c.args)} -> ${out.slice(0, 600)}`);
    });
  }
  mkdirSync(transcriptDir, { recursive: true });
  writeFileSync(new URL("js.txt", transcriptDir), lines.join(""));
  assert.deepEqual(failures, []);
});
