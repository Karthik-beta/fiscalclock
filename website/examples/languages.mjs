// Every example is written once per language. This table says where each
// language's source and expected output live, relative to examples/, and is
// read by the pages that show the examples and by scripts/test-examples.mjs.
// The order is the order of the tabs; Python comes first (DD-034).
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
