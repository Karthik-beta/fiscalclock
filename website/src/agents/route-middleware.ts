import { defineRouteMiddleware } from "@astrojs/starlight/route-data";
import { markdownCopyOf } from "./markdown-copy.ts";

/** Points agents from each docs page to its Markdown copy. */
export const onRequest = defineRouteMiddleware(async ({ locals: { starlightRoute } }) => {
  const href = await markdownCopyOf(starlightRoute.entry.id);
  if (!href) return;
  starlightRoute.head.push({
    tag: "link",
    attrs: { rel: "alternate", type: "text/markdown", href },
  });
});
