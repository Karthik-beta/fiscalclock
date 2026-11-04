import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { expect, test } from "@playwright/test";
import { importDocs } from "../scripts/import-docs.mjs";

const IMPORTS = {
  "docs/a.md": { route: "/a/", description: "Page A." },
  "docs/b.md": { route: "/guide/b/", description: "Page B." },
};

function fixture(aBody: string): { repo: string; website: string } {
  const repo = mkdtempSync(join(tmpdir(), "import-docs-"));
  mkdirSync(join(repo, "docs"));
  mkdirSync(join(repo, "conformance", "vectors"), { recursive: true });
  writeFileSync(join(repo, "LICENSE-MIT"), "licence");
  writeFileSync(join(repo, "docs", "a.md"), `# Title of A\n\n${aBody}\n`);
  writeFileSync(join(repo, "docs", "b.md"), "# B\n\n## Some heading\n");
  return { repo, website: join(repo, "website") };
}

test("a relative link to a missing path stops the import and writes nothing", () => {
  const { repo, website } = fixture("Fine: [b](b.md).\n\nBroken: [gone](../spec/missing.md#x).");
  expect(() => importDocs(repo, website, IMPORTS)).toThrow("docs/a.md:5: ../spec/missing.md#x");
  expect(existsSync(join(website, "src"))).toBe(false);
});

test("links become site routes or GitHub URLs, and code is left alone", () => {
  const { repo, website } = fixture(
    [
      "[b](b.md#some-heading) [self](#top) [lic](../LICENSE-MIT) [vec](../conformance/vectors/) [web](https://example.com/x.md)",
      "",
      "[ref]: ./b.md",
      "",
      "`[not a link](nowhere.md)`",
      "",
      "```md",
      "[also not a link](nowhere.md)",
      "```",
    ].join("\n"),
  );
  importDocs(repo, website, IMPORTS);
  const page = readFileSync(join(website, "src/content/docs/a.md"), "utf8");
  expect(page).toContain('title: "Title of A"');
  expect(page).toContain('description: "Page A."');
  expect(page).not.toContain("# Title of A");
  expect(page).toContain(
    "[b](/guide/b/#some-heading) [self](#top) " +
      "[lic](https://github.com/Karthik-beta/fiscalclock/blob/main/LICENSE-MIT) " +
      "[vec](https://github.com/Karthik-beta/fiscalclock/tree/main/conformance/vectors) " +
      "[web](https://example.com/x.md)",
  );
  expect(page).toContain("[ref]: /guide/b/");
  expect(page).toContain("`[not a link](nowhere.md)`");
  expect(page).toContain("[also not a link](nowhere.md)");
  expect(readFileSync(join(website, "src/content/docs/guide/b.md"), "utf8")).toContain("## Some heading");
});
