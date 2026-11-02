import { defineConfig } from "astro/config";
import sitemap from "@astrojs/sitemap";
import starlight from "@astrojs/starlight";
import starlightLlmsTxt from "starlight-llms-txt";
import { PAGE_SEPARATOR, llmsPostbuild } from "./src/agents/llms-postbuild.ts";

const site = "https://fiscalclock.karthik.nz";
const repo = "https://github.com/Karthik-beta/fiscalclock";
// Pages that ask search engines not to index them (robots noindex) stay out of the sitemap.
const unlisted = new Set([`${site}/guides/style-check/`]);

export default defineConfig({
  site,
  output: "static",
  integrations: [
    starlight({
      title: "fiscalclock",
      description: "Fiscal calendar engine: compute any fiscal calendar from a JSON definition.",
      logo: {
        light: "./src/assets/brand/fiscalclock-lockup.svg",
        dark: "./src/assets/brand/fiscalclock-lockup-dark.svg",
        alt: "",
        replacesTitle: true,
      },
      favicon: "/favicon.svg",
      head: [
        { tag: "link", attrs: { rel: "icon", href: "/favicon.ico", sizes: "32x32" } },
        { tag: "link", attrs: { rel: "apple-touch-icon", href: "/apple-touch-icon.png" } },
        { tag: "meta", attrs: { property: "og:image", content: `${site}/og-image.png` } },
        { tag: "meta", attrs: { property: "og:image:width", content: "1200" } },
        { tag: "meta", attrs: { property: "og:image:height", content: "630" } },
        { tag: "meta", attrs: { property: "og:image:alt", content: "fiscalclock" } },
      ],
      social: [{ icon: "github", label: "GitHub", href: repo }],
      plugins: [
        starlightLlmsTxt({
          pageSeparator: PAGE_SEPARATOR,
          // Each heading's anchor otherwise becomes a "Section titled" link line.
          customSelectors: { all: [".sl-anchor-link"] },
        }),
      ],
      routeMiddleware: "./src/agents/route-middleware.ts",
      components: { PageTitle: "./src/components/PageTitle.astro" },
      // src/pages/404.astro replaces Starlight's 404 route, which looks up a docs entry named 404.
      disable404Route: true,
      editLink: { baseUrl: `${repo}/edit/main/website/` },
      customCss: [
        "./src/styles/generated/tokens.css",
        "./src/styles/fonts.css",
        "./src/styles/theme.css",
      ],
      // Pages are listed by slug, in reading order. A new page needs an entry here.
      sidebar: [
        {
          label: "Start here",
          items: ["start-here/install", "start-here/quick-start", { label: "Playground", link: "/playground/" }],
        },
        {
          label: "Guides",
          items: [
            "guides/write-a-definition",
            "guides/week-based-years",
            "guides/labels",
            "guides/dates-and-intervals",
            "guides/date-dimension",
          ],
        },
        {
          label: "Reference",
          items: [
            { label: "API reference", slug: "reference/api" },
            { label: "Definition format spec", slug: "reference/spec/definition-v1" },
            { label: "API spec", slug: "reference/spec/api-v1" },
          ],
        },
        {
          label: "Project",
          items: [
            "project/changelog",
            { label: "GitHub", link: repo },
            { label: "Design decisions (ADRs)", link: `${repo}/wiki/ADR-Index` },
            { label: "Roadmap", link: `${repo}/wiki/Roadmap` },
          ],
        },
      ],
    }),
    llmsPostbuild(),
    // Starlight adds its own sitemap only when this integration is absent, so the filter takes effect.
    sitemap({ filter: (page) => !unlisted.has(page) }),
  ],
});
