export const SITE = "https://fiscalclock.karthik.nz";

/**
 * Docs pages that ask search engines not to index them. They stay out of the sitemap and
 * of llms.txt, llms-full.txt and llms-small.txt, and still get a Markdown copy.
 */
export const HIDDEN_PAGES = ["guides/style-check"];

/** Starlight gives its generated 404 page this entry id; it has no Markdown copy. */
export function hasMarkdownCopy(id: string): boolean {
  return id !== "404" && !id.endsWith("/404");
}

// The home page's id is "index" in the docs collection and "" in Starlight's route data.
const isHome = (id: string) => id === "" || id === "index";

/** Root-relative URL of a docs entry's HTML page. */
export function pagePath(id: string): string {
  return isHome(id) ? "/" : `/${id}/`;
}

/** Root-relative URL of a docs entry's Markdown copy. */
export function markdownPath(id: string): string {
  return isHome(id) ? "/index.md" : `/${id}.md`;
}
