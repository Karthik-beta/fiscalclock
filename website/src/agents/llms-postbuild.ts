import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join, sep } from "node:path";
import { fileURLToPath } from "node:url";
import type { AstroIntegration } from "astro";
import { parse } from "yaml";
import { HIDDEN_PAGES, SITE, markdownPath } from "./paths.ts";

/**
 * Set as starlight-llms-txt's `pageSeparator`, so that the build step below can
 * find page boundaries in llms-full.txt and llms-small.txt. It is replaced by a
 * blank line, so it never reaches a published file.
 */
export const PAGE_SEPARATOR = "\n\n<!-- fiscalclock:page -->\n\n";

interface MarkdownPage {
  id: string;
  title: string;
  description?: string;
}

/**
 * Finishes the files starlight-llms-txt writes. The plugin cannot leave a page
 * out of llms-full.txt and does not list pages in llms.txt, so after the build
 * this drops the HIDDEN_PAGES pages from both concatenated files and adds a
 * list of every page's Markdown copy to llms.txt.
 */
export function llmsPostbuild(): AstroIntegration {
  let docs = "";
  return {
    name: "fiscalclock-llms-postbuild",
    hooks: {
      "astro:config:done": ({ config }) => {
        docs = fileURLToPath(new URL("src/content/docs/", config.root));
      },
      "astro:build:done": ({ dir }) => {
        const dist = fileURLToPath(dir);
        const pages = markdownPages(dist, docs);
        const excluded = new Set(pages.filter((page) => HIDDEN_PAGES.includes(page.id)).map((page) => page.title));
        for (const id of HIDDEN_PAGES) {
          if (!pages.some((page) => page.id === id)) throw new Error(`llms: excluded page ${id} has no Markdown copy`);
        }
        for (const file of ["llms-full.txt", "llms-small.txt"]) {
          edit(join(dist, file), (text) => dropPages(text, excluded, file));
        }
        edit(join(dist, "llms.txt"), (text) => addPageList(text, pages.filter((page) => !HIDDEN_PAGES.includes(page.id))));
      },
    },
  };
}

function markdownPages(dist: string, docs: string): MarkdownPage[] {
  return readdirSync(dist, { recursive: true, encoding: "utf8" })
    .filter((file) => file.endsWith(".md"))
    .map((file) => {
      const [heading] = readFileSync(join(dist, file), "utf8").split("\n");
      const id = file.split(sep).join("/").replace(/\.md$/, "");
      if (!heading?.startsWith("# ")) throw new Error(`llms: ${file} does not start with a title`);
      return { id, title: heading.slice(2), description: frontmatterDescription(docs, id) };
    })
    .sort((a, b) => (a.id === "index" ? -1 : b.id === "index" ? 1 : a.id.localeCompare(b.id)));
}

/** The `description` in the frontmatter of the docs page with this id, if it has one. */
function frontmatterDescription(docs: string, id: string): string | undefined {
  const source = [".mdx", ".md"].map((ext) => join(docs, id + ext)).find((path) => existsSync(path));
  if (!source) throw new Error(`llms: no docs source file for the page ${id}`);
  const frontmatter = /^---\r?\n([\s\S]*?)\r?\n---/.exec(readFileSync(source, "utf8"));
  const description = frontmatter ? (parse(frontmatter[1]!) as { description?: unknown } | null)?.description : undefined;
  return typeof description === "string" ? description : undefined;
}

function dropPages(text: string, titles: Set<string>, file: string): string {
  const sections = text.split(PAGE_SEPARATOR);
  const kept = sections.filter((section) => !titles.has(section.split("\n")[0]!.replace(/^# /, "")));
  if (sections.length - kept.length !== titles.size) {
    throw new Error(`llms: expected to drop ${titles.size} pages from ${file}, dropped ${sections.length - kept.length}`);
  }
  return kept.join("\n\n");
}

function addPageList(text: string, pages: MarkdownPage[]): string {
  const list = pages
    .map(({ id, title, description }) => `- [${title}](${SITE}${markdownPath(id)})${description ? `: ${description}` : ""}`)
    .join("\n");
  const section = `## Pages\n\nEach page of the documentation as Markdown.\n\n${list}\n\n`;
  const at = text.indexOf("## Notes");
  if (at === -1) throw new Error("llms: llms.txt has no Notes section to put the page list before");
  return text.slice(0, at) + section + text.slice(at);
}

function edit(file: string, change: (text: string) => string): void {
  writeFileSync(file, change(readFileSync(file, "utf8")));
}
