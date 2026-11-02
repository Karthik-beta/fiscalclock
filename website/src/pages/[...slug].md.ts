import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import type { APIRoute, GetStaticPaths, InferGetStaticPropsType } from "astro";
import { getCollection } from "astro:content";
import { mdxToMarkdown } from "../agents/mdx-to-markdown.ts";
import { SITE, hasMarkdownCopy, pagePath } from "../agents/paths.ts";

export const getStaticPaths = (async () => {
  const entries = await getCollection(
    "docs",
    (entry) => hasMarkdownCopy(entry.id) && !(import.meta.env.PROD && entry.data.draft),
  );
  return entries.map((entry) => ({ params: { slug: entry.id }, props: { entry } }));
}) satisfies GetStaticPaths;

type Props = InferGetStaticPropsType<typeof getStaticPaths>;

export const GET: APIRoute<Props> = ({ props: { entry } }) => {
  if (!entry.filePath) throw new Error(`${entry.id}: the docs entry has no source file`);
  const sourceDir = dirname(resolve(entry.filePath));
  const markdown = mdxToMarkdown({
    id: entry.id,
    title: entry.data.title,
    description: entry.data.description,
    body: entry.body ?? "",
    url: new URL(pagePath(entry.id), SITE).href,
    readRaw: (specifier) => readFileSync(resolve(sourceDir, specifier), "utf8"),
  });
  return new Response(markdown, { headers: { "Content-Type": "text/markdown; charset=utf-8" } });
};
