import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { expect, test } from "@playwright/test";

// Runs scripts/should-build.sh in throwaway Git repositories laid out like this
// one, from website/ as Vercel does. Exit 1 means build, exit 0 means skip.

const script = join(import.meta.dirname, "..", "scripts", "should-build.sh");
const BUILD = 1;
const SKIP = 0;

// Hermetic Git: no user or system config, so signing or hooks settings on the
// machine running the tests cannot change the result.
const env = {
  ...process.env,
  GIT_CONFIG_GLOBAL: join(tmpdir(), "should-build-empty-gitconfig"),
  GIT_CONFIG_NOSYSTEM: "1",
  GIT_AUTHOR_NAME: "test",
  GIT_AUTHOR_EMAIL: "test@example.invalid",
  GIT_COMMITTER_NAME: "test",
  GIT_COMMITTER_EMAIL: "test@example.invalid",
};
writeFileSync(env.GIT_CONFIG_GLOBAL, "");

function git(cwd: string, ...args: string[]): string {
  const result = spawnSync("git", args, { cwd, env, encoding: "utf8" });
  if (result.status !== 0) throw new Error(`git ${args.join(" ")}: ${result.stderr}`);
  return result.stdout.trim();
}

function commit(repo: string, file: string): string {
  const path = join(repo, file);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${Math.random()}\n`);
  git(repo, "add", "-A");
  git(repo, "commit", "--quiet", "-m", `change ${file}`);
  return git(repo, "rev-parse", "HEAD");
}

function newRepo(): string {
  const repo = mkdtempSync(join(tmpdir(), "should-build-"));
  git(repo, "init", "--quiet", "--initial-branch=main");
  commit(repo, "website/package.json");
  return repo;
}

function shouldBuild(repo: string, previousSha?: string): number | null {
  const vars = previousSha === undefined ? env : { ...env, VERCEL_GIT_PREVIOUS_SHA: previousSha };
  return spawnSync("sh", [script], { cwd: join(repo, "website"), env: vars, encoding: "utf8" }).status;
}

const repos: string[] = [];
test.afterAll(() => repos.forEach((repo) => rmSync(repo, { recursive: true, force: true })));
function tracked(repo: string): string {
  repos.push(repo);
  return repo;
}

const LAST_COMMIT_CASES: [string, number][] = [
  ["website/src/content/docs/index.mdx", BUILD],
  ["website/vercel.json", BUILD],
  ["design/tokens/colour.json", BUILD],
  ["assets/brand/logo/mark.svg", BUILD],
  ["spec/definition-v1.md", BUILD],
  ["CHANGELOG.md", BUILD],
  ["conformance/vectors/cited/kroger.json", BUILD],
  ["bindings/js/package.json", BUILD],
  ["crates/core/src/lib.rs", SKIP],
  ["bindings/js/src/index.ts", SKIP],
  ["bindings/python/pyproject.toml", SKIP],
  ["conformance/vectors/generated/iso.json", SKIP],
  ["README.md", SKIP],
];

for (const [file, expected] of LAST_COMMIT_CASES) {
  test(`with no previous deployment, a last commit touching ${file} ${expected === BUILD ? "builds" : "skips"}`, () => {
    const repo = tracked(newRepo());
    commit(repo, file);
    expect(shouldBuild(repo)).toBe(expected);
  });
}

test("the first commit of a repository builds, because HEAD^ does not exist", () => {
  const repo = tracked(newRepo());
  expect(shouldBuild(repo)).toBe(BUILD);
});

test("a shallow clone with no parent commit builds", () => {
  const origin = tracked(newRepo());
  commit(origin, "crates/core/src/lib.rs");
  const clone = tracked(mkdtempSync(join(tmpdir(), "should-build-clone-")));
  git(clone, "clone", "--quiet", "--depth=1", `file://${origin.replaceAll("\\", "/")}`, ".");
  expect(shouldBuild(clone)).toBe(BUILD);
});

test("compares with the previous deployment, not only the last commit", () => {
  const repo = tracked(newRepo());
  const deployed = commit(repo, "crates/core/src/lib.rs");
  commit(repo, "website/src/content/docs/index.mdx");
  commit(repo, "crates/core/src/lib.rs");
  expect(shouldBuild(repo), "HEAD^ alone sees only the last commit").toBe(SKIP);
  expect(shouldBuild(repo, deployed)).toBe(BUILD);
});

test("skips when nothing the site reads changed since the previous deployment", () => {
  const repo = tracked(newRepo());
  const deployed = commit(repo, "website/src/content/docs/index.mdx");
  commit(repo, "crates/core/src/lib.rs");
  commit(repo, "bindings/python/pyproject.toml");
  expect(shouldBuild(repo, deployed)).toBe(SKIP);
});

test("a previous deployment outside the shallow clone builds", () => {
  const repo = tracked(newRepo());
  commit(repo, "crates/core/src/lib.rs");
  expect(shouldBuild(repo, "0123456789abcdef0123456789abcdef01234567")).toBe(BUILD);
});
