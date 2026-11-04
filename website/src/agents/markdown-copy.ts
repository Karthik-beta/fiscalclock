import { getEntry } from "astro:content";
import { hasMarkdownCopy, markdownPath } from "./paths.ts";

/**
 * The Markdown copy's URL for a page, or undefined when the page has none.
 * Custom pages such as the playground render through StarlightPage with a
 * route entry id but are not in the docs collection, so no copy is generated.
 */
export async function markdownCopyOf(id: string): Promise<string | undefined> {
  if (!hasMarkdownCopy(id)) return undefined;
  const entry = await getEntry("docs", id === "" ? "index" : id);
  return entry ? markdownPath(id) : undefined;
}
