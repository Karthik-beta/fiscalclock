// Runs every example on the website in every language against the published
// packages, and fails unless each prints exactly its committed `.out` file.
// The pages show those same files, so a snippet or output that is wrong on
// the site fails here.
//
// Python runs in website/.venv with the pinned examples/python/requirements.txt,
// JavaScript uses the fiscalclock package pinned in website/package.json, and
// Rust builds examples/rust against the crate pinned in its Cargo.lock.
//
// Each example runs in the DEFINITIONS folder, so it opens a definition by its
// bare file name, as a reader who saved the file next to their script would.

import { spawnSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { DEFINITIONS, LANGUAGES, outputPath, sourcePath } from "../examples/languages.mjs";

const website = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const examples = join(website, "examples");
const definitions = resolve(website, "..", DEFINITIONS);
const windows = process.platform === "win32";

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: examples,
    encoding: "utf8",
    env: { ...process.env, PYTHONUTF8: "1" },
    ...options,
  });
  if (result.error) throw new Error(`${command} could not start: ${result.error.message}`);
  return result;
}

function check(result, what) {
  if (result.status !== 0) throw new Error(`${what} failed:\n${result.stdout}${result.stderr}`);
  return result.stdout;
}

function findPython() {
  for (const [command, ...args] of [["python3"], ["python"], ["py", "-3"]]) {
    const probe = spawnSync(command, [...args, "--version"], { encoding: "utf8" });
    if (probe.status === 0) return [command, ...args];
  }
  throw new Error("no Python interpreter found on PATH");
}

// How to prepare each language once. Each entry returns the fiscalclock
// version under test and a function that runs one example by name.
const RUNNERS = {
  python(language) {
    const venv = join(website, ".venv");
    const python = join(venv, windows ? "Scripts/python.exe" : "bin/python");
    if (!existsSync(python)) {
      const [command, ...args] = findPython();
      check(run(command, [...args, "-m", "venv", venv]), "creating website/.venv");
    }
    check(
      run(python, ["-m", "pip", "install", "--quiet", "--disable-pip-version-check", "-r", "python/requirements.txt"]),
      "pip install -r examples/python/requirements.txt",
    );
    const version = check(run(python, ["-c", "import fiscalclock; print(fiscalclock.__version__)"]), "importing fiscalclock").trim();
    return { version, run: (name) => run(python, ["-I", join(examples, sourcePath(language, name))], { cwd: definitions }) };
  },
  js(language) {
    const pkg = JSON.parse(readFileSync(join(website, "node_modules/fiscalclock/package.json"), "utf8"));
    return { version: pkg.version, run: (name) => run(process.execPath, [join(examples, sourcePath(language, name))], { cwd: definitions }) };
  },
  rust() {
    const manifest = join(examples, "rust/Cargo.toml");
    check(run("cargo", ["build", "--quiet", "--locked", "--bins", "--manifest-path", manifest]), "cargo build");
    const lock = readFileSync(join(examples, "rust/Cargo.lock"), "utf8");
    const version = lock.match(/name = "fiscalclock"\nversion = "([^"]+)"/)[1];
    const bin = (name) => join(examples, "rust/target/debug", windows ? `${name}.exe` : name);
    return { version, run: (name) => run(bin(name), [], { cwd: definitions }) };
  },
};

const normalise = (text) => text.replace(/\r\n/g, "\n");

// Every example must exist in every language, with an expected output.
const names = new Set();
for (const language of LANGUAGES) {
  for (const file of readdirSync(join(examples, language.dir))) {
    if (file.endsWith(language.ext)) names.add(file.slice(0, -language.ext.length));
  }
}
const missing = [];
for (const name of names) {
  for (const language of LANGUAGES) {
    for (const path of [sourcePath(language, name), outputPath(language, name)]) {
      if (!existsSync(join(examples, path))) missing.push(path);
    }
  }
}
if (missing.length > 0) {
  console.error(`test-examples: these files are missing:\n  ${missing.join("\n  ")}`);
  process.exit(1);
}

let failures = 0;
for (const language of LANGUAGES) {
  const runner = RUNNERS[language.key](language);
  let passed = 0;
  for (const name of [...names].sort()) {
    const result = runner.run(name);
    const expected = normalise(readFileSync(join(examples, outputPath(language, name)), "utf8"));
    const actual = normalise(result.stdout);
    if (result.status === 0 && actual === expected) {
      passed += 1;
      continue;
    }
    failures += 1;
    console.error(`FAIL ${sourcePath(language, name)} (exit ${result.status})`);
    console.error(`--- expected (${outputPath(language, name)})\n${expected}--- actual\n${actual}${result.stderr}`);
  }
  console.log(`${language.label}: fiscalclock ${runner.version}, ${passed} of ${names.size} examples print their expected output`);
}
process.exit(failures === 0 ? 0 : 1);
