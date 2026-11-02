import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join, sep } from "node:path";
import { fileURLToPath } from "node:url";
import type { AstroIntegration } from "astro";
import { LLMS_EXCLUDE, SITE, markdownPath } from "./paths.ts";

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
 * this drops the LLMS_EXCLUDE pages from both concatenated files and adds a
 * list of every page's Markdown copy to llms.txt.
 */
export function llmsPostbuild(): AstroIntegration {
  return {
    name: "fiscalclock-llms-postbuild",
    hooks: {
      "astro:build:done": ({ dir }) => {
        const dist = fileURLToPath(dir);
        const pages = markdownPages(dist);
        const excluded = new Set(pages.filter((page) => LLMS_EXCLUDE.includes(page.id)).map((page) => page.title));
        for (const id of LLMS_EXCLUDE) {
          if (!pages.some((page) => page.id === id)) throw new Error(`llms: excluded page ${id} has no Markdown copy`);
        }
        for (const file of ["llms-full.txt", "llms-small.txt"]) {
          edit(join(dist, file), (text) => dropPages(text, excluded, file));
        }
        edit(join(dist, "llms.txt"), (text) => addPageList(text, pages.filter((page) => !LLMS_EXCLUDE.includes(page.id))));
      },
    },
  };
}

function markdownPages(dist: string): MarkdownPage[] {
  return readdirSync(dist, { recursive: true, encoding: "utf8" })
    .filter((file) => file.endsWith(".md"))
    .map((file) => {
      const [heading, , second] = readFileSync(join(dist, file), "utf8").split("\n");
      const id = file.split(sep).join("/").replace(/\.md$/, "");
      if (!heading?.startsWith("# ")) throw new Error(`llms: ${file} does not start with a title`);
      // The description, when the page has one, is the line after the title.
      const description = second && /^[\p{L}\p{N}]/u.test(second) ? second : undefined;
      return { id, title: heading.slice(2), description };
    })
    .sort((a, b) => (a.id === "index" ? -1 : b.id === "index" ? 1 : a.id.localeCompare(b.id)));
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
