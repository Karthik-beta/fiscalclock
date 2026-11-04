import { expect, test } from "@playwright/test";
import { inlineScriptHashes } from "../scripts/csp-hashes.mjs";
import { applyRoutes, compileRoutes, loadConfig } from "../scripts/serve-with-headers.mjs";

// These tests compile vercel.json with Vercel's own route compiler and assert
// what a visitor would receive, so a later edit cannot silently drop a rule.

const PRODUCTION = "fiscalclock.karthik.nz";
const config = loadConfig();
const routes = compileRoutes(config);
const respond = (pathname: string, host = PRODUCTION) => applyRoutes(routes, { pathname, host });
const headersFor = (pathname: string) => respond(pathname).headers as Record<string, string>;

const SAMPLE_PATHS = [
  "/",
  "/guides/style-check/",
  "/404.html",
  "/_astro/page.cNngjUgL.js",
  "/fonts/mona-sans/mona-sans-latin-wght-normal.woff2",
  "/pagefind/pagefind.js",
  "/playground/fiscalclock_bg.wasm",
  "/guides/style-check.md",
  "/llms.txt",
];

function directives(policy: string): Map<string, string[]> {
  return new Map(
    policy
      .split(";")
      .map((d) => d.trim().split(/\s+/))
      .filter(([name]) => name)
      .map(([name, ...sources]) => [name, sources]),
  );
}

test("every response carries the security headers", () => {
  for (const path of SAMPLE_PATHS) {
    const headers = headersFor(path);
    expect(headers["X-Content-Type-Options"], path).toBe("nosniff");
    expect(headers["Referrer-Policy"], path).toBe("strict-origin-when-cross-origin");
    expect(headers["Permissions-Policy"], path).toMatch(/camera=\(\)/);
    expect(headers["Content-Security-Policy"], path).toBeTruthy();
  }
});

test("the Content-Security-Policy allows only this origin, hashed inline scripts and WebAssembly", () => {
  const csp = directives(headersFor("/")["Content-Security-Policy"]);
  expect(csp.get("default-src")).toEqual(["'self'"]);
  expect(csp.get("img-src")).toEqual(["'self'", "data:"]);
  expect(csp.get("object-src")).toEqual(["'none'"]);
  expect(csp.get("base-uri")).toEqual(["'self'"]);
  expect(csp.get("form-action")).toEqual(["'self'"]);
  expect(csp.get("frame-ancestors")).toEqual(["'none'"]);

  const scriptSrc = csp.get("script-src") ?? [];
  expect(scriptSrc).toContain("'wasm-unsafe-eval'");
  expect(scriptSrc).not.toContain("'unsafe-inline'");
  expect(scriptSrc).not.toContain("'unsafe-eval'");

  const sources = [...csp.values()].flat();
  const allowed = /^('self'|'none'|'unsafe-inline'|'wasm-unsafe-eval'|'sha256-[A-Za-z0-9+/]+=*'|data:)$/;
  expect(sources.filter((s) => !allowed.test(s)), "no third-party origins or schemes").toEqual([]);
});

test("script-src lists exactly the inline scripts in the built site", () => {
  const csp = directives(headersFor("/")["Content-Security-Policy"]);
  const listed = (csp.get("script-src") ?? []).filter((s) => s.startsWith("'sha256-")).sort();
  expect(listed, "run `node scripts/csp-hashes.mjs` after `npm run build` and commit vercel.json").toEqual(
    inlineScriptHashes(),
  );
});

test("WebAssembly, Markdown copies and llms files get their content types", () => {
  expect(headersFor("/playground/fiscalclock_bg.wasm")["Content-Type"]).toBe("application/wasm");
  expect(headersFor("/guides/style-check.md")["Content-Type"]).toBe("text/markdown; charset=utf-8");
  expect(headersFor("/index.md")["Content-Type"]).toBe("text/markdown; charset=utf-8");
  for (const file of ["/llms.txt", "/llms-full.txt", "/llms-small.txt"]) {
    expect(headersFor(file)["Content-Type"], file).toBe("text/plain; charset=utf-8");
  }
  expect(headersFor("/fonts/mona-sans/LICENSE.txt")["Content-Type"]).toBeUndefined();
});

test("hashed assets are immutable, fonts are cached briefly, pages revalidate", () => {
  expect(headersFor("/_astro/page.cNngjUgL.js")["Cache-Control"]).toBe("public, max-age=31536000, immutable");
  const font = headersFor("/fonts/mona-sans/mona-sans-latin-wght-normal.woff2")["Cache-Control"];
  expect(font).toMatch(/max-age=\d+/);
  expect(font).not.toMatch(/immutable/);
  // No rule, so Vercel's default "public, max-age=0, must-revalidate" applies.
  for (const page of ["/", "/guides/style-check/", "/404.html", "/guides/style-check.md", "/llms.txt"]) {
    expect(headersFor(page)["Cache-Control"], page).toBeUndefined();
  }
});

test("the production vercel.app host redirects permanently to the custom domain, path kept", () => {
  for (const path of ["/", "/guides/style-check/", "/llms.txt"]) {
    expect(respond(path, "fiscalclock.vercel.app").redirect).toEqual({
      status: 308,
      location: `https://${PRODUCTION}${path}`,
    });
  }
});

test("preview deployments and the custom domain are not redirected to another host", () => {
  for (const host of [PRODUCTION, "fiscalclock-git-website-karthik-beta.vercel.app", "fiscalclock-a1b2c3d4e-karthik-beta.vercel.app"]) {
    expect(respond("/guides/style-check/", host).redirect, host).toBeUndefined();
  }
});

test("each page has one URL: the trailing-slash form Astro builds", () => {
  expect(respond("/guides/style-check").redirect).toEqual({ status: 308, location: "/guides/style-check/" });
  expect(respond("/guides/style-check/").redirect).toBeUndefined();
  expect(respond("/guides/style-check.md").redirect).toBeUndefined();
  expect(respond("/llms.txt").redirect).toBeUndefined();
  expect(config.cleanUrls).toBeUndefined();
});

test("Vercel installs without install scripts and asks should-build.sh whether to build", () => {
  expect(config.installCommand).toBe("npm ci --ignore-scripts");
  expect(config.ignoreCommand).toBe("sh scripts/should-build.sh");
});
