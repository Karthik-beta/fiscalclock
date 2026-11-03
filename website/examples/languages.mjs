// Every example is written once per language. This table says where each
// language's source and expected output live, relative to examples/. Only
// scripts/test-examples.mjs reads it (and tests/build-inputs.spec.ts reads
// DEFINITIONS); the pages import their example files directly and list their
// tabs themselves. Python comes first in both (DD-034).
export const LANGUAGES = [
  { key: "python", label: "Python", lang: "python", dir: "python", ext: ".py" },
  { key: "js", label: "JavaScript", lang: "js", dir: "js", ext: ".mjs" },
  { key: "rust", label: "Rust", lang: "rust", dir: "rust/src/bin", ext: ".rs" },
];

/** The source file of example `name` in `language`, relative to examples/. */
export function sourcePath(language, name) {
  return `${language.dir}/${name}${language.ext}`;
}

/** The exact output that example `name` prints in `language`, relative to examples/. */
export function outputPath(language, name) {
  return `${language.dir}/${name}.out`;
}

/**
 * The folder, relative to the repository root, that holds the definitions the
 * examples open by bare file name. Example definitions live in docs/ (AGENTS.md,
 * design rule 1), and scripts/test-examples.mjs runs every example from there.
 */
export const DEFINITIONS = "docs/examples";
