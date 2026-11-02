import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

const site = "https://fiscalclock.karthik.nz";
const repo = "https://github.com/Karthik-beta/fiscalclock";

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
      editLink: { baseUrl: `${repo}/edit/main/website/` },
      customCss: [
        "./src/styles/generated/tokens.css",
        "./src/styles/fonts.css",
        "./src/styles/theme.css",
      ],
      // Each group lists the pages in its folder under src/content/docs/, so
      // later work adds a page by adding a file.
      sidebar: [
        { label: "Start here", items: [{ autogenerate: { directory: "start-here" } }] },
        { label: "Guides", items: [{ autogenerate: { directory: "guides" } }] },
        { label: "Reference", items: [{ autogenerate: { directory: "reference" } }] },
        { label: "Project", items: [{ autogenerate: { directory: "project" } }] },
      ],
    }),
  ],
});
