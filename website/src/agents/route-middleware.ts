import { defineRouteMiddleware } from "@astrojs/starlight/route-data";
import { hasMarkdownCopy, markdownPath } from "./paths.ts";

/** Points agents from each docs page to its Markdown copy. */
export const onRequest = defineRouteMiddleware(({ locals: { starlightRoute } }) => {
  const { id } = starlightRoute.entry;
  if (!hasMarkdownCopy(id)) return;
  starlightRoute.head.push({
    tag: "link",
    attrs: { rel: "alternate", type: "text/markdown", href: markdownPath(id) },
  });
});
