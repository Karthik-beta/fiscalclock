import { readFileSync } from "node:fs";
import { posix } from "node:path";
import type { APIRoute, GetStaticPaths, InferGetStaticPropsType } from "astro";
import { getCollection } from "astro:content";
import { mdxToMarkdown } from "../agents/mdx-to-markdown.ts";
import { SITE, hasMarkdownCopy, pagePath } from "../agents/paths.ts";

// The modules a page may import values from, keyed by path from the project root.
const MODULES = import.meta.glob<Record<string, unknown>>("/src/lib/*.ts");

export const getStaticPaths = (async () => {
  const entries = await getCollection(
    "docs",
    (entry) => hasMarkdownCopy(entry.id) && !(import.meta.env.PROD && entry.data.draft),
  );
  return entries.map((entry) => ({ params: { slug: entry.id }, props: { entry } }));
}) satisfies GetStaticPaths;

type Props = InferGetStaticPropsType<typeof getStaticPaths>;

export const GET: APIRoute<Props> = async ({ props: { entry } }) => {
  if (!entry.filePath) throw new Error(`${entry.id}: the docs entry has no source file`);
  // filePath is relative to the project root, which is the working directory of a build.
  const sourceDir = posix.dirname(entry.filePath.replaceAll("\\", "/"));
  const markdown = await mdxToMarkdown({
    id: entry.id,
    title: entry.data.title,
    description: entry.data.description,
    body: entry.body ?? "",
    format: entry.filePath.endsWith(".mdx") ? "mdx" : "md",
    url: new URL(pagePath(entry.id), SITE).href,
    readRaw: (specifier) => readFileSync(posix.join(sourceDir, specifier), "utf8"),
    readModule: (specifier) => {
      const path = "/" + posix.join(sourceDir, specifier);
      const load = MODULES[path] ?? MODULES[`${path}.ts`];
      if (!load) throw new Error(`${entry.id}: ${specifier} is not a module in src/lib/`);
      return load();
    },
  });
  return new Response(markdown, { headers: { "Content-Type": "text/markdown; charset=utf-8" } });
};
