// A static server for tests only: serves dist/ with the redirects and headers
// in vercel.json, compiled to routes by @vercel/routing-utils, the package
// Vercel publishes for that translation. It is not Vercel: it handles only the
// route fields vercel.json produces here and throws on anything else.
//
//   node scripts/serve-with-headers.mjs [port]

import { existsSync, readFileSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { dirname, extname, join, normalize, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import routingUtils from "@vercel/routing-utils";

const website = resolve(dirname(fileURLToPath(import.meta.url)), "..");

// Vercel's documented default when no rule sets Cache-Control:
// https://vercel.com/docs/caching/cache-control-headers#default-cache-control-value
const DEFAULT_CACHE_CONTROL = "public, max-age=0, must-revalidate";

// Deliberately no entry for .wasm or .md: their types must come from vercel.json.
const TYPES = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".json": "application/json",
  ".xml": "application/xml",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".ico": "image/x-icon",
  ".woff2": "font/woff2",
  ".txt": "text/plain",
};

export function loadConfig(file = join(website, "vercel.json")) {
  return JSON.parse(readFileSync(file, "utf8"));
}

export function compileRoutes(config) {
  const { routes, error } = routingUtils.getTransformedRoutes(config);
  if (error) throw new Error(`vercel.json: ${error.message}`);
  for (const route of routes) {
    const unknown = Object.keys(route).filter((k) => !["src", "headers", "status", "continue", "has"].includes(k));
    if (unknown.length > 0) throw new Error(`serve-with-headers: unsupported route fields ${unknown.join(", ")}`);
  }
  return routes;
}

function hostMatches(condition, host) {
  if (condition.type !== "host") throw new Error(`serve-with-headers: unsupported has type ${condition.type}`);
  const { value } = condition;
  if (typeof value === "object" && Object.keys(value).join() === "eq") return host === value.eq;
  throw new Error(`serve-with-headers: unsupported host condition ${JSON.stringify(value)}`);
}

/** What Vercel's routes decide for a request before the filesystem: a redirect, or headers to add. */
export function applyRoutes(routes, { pathname, host }) {
  const headers = {};
  for (const route of routes) {
    const match = new RegExp(route.src).exec(pathname);
    if (!match) continue;
    if (route.has && !route.has.every((c) => hostMatches(c, host))) continue;
    if (route.status) {
      const location = route.headers.Location.replace(/\$(\d)/g, (_, i) => match[Number(i)] ?? "");
      return { redirect: { status: route.status, location }, headers };
    }
    if (route.headers) Object.assign(headers, route.headers);
    if (!route.continue) break;
  }
  return { headers };
}

function resolveFile(roots, pathname) {
  for (const [prefix, root] of roots) {
    if (!pathname.startsWith(prefix)) continue;
    let rel = decodeURIComponent(pathname.slice(prefix.length));
    if (rel === "" || rel.endsWith("/")) rel += "index.html";
    const file = normalize(join(root, rel));
    if (!file.startsWith(normalize(root) + sep)) return null;
    if (existsSync(file) && statSync(file).isFile()) return file;
  }
  return null;
}

/**
 * Serves `root` with vercel.json's routes. `mounts` maps a URL prefix such as
 * "/__fixture/" to an extra directory, for test pages that are not in dist/.
 */
export function serve({ root = join(website, "dist"), config = loadConfig(), mounts = {} } = {}) {
  const routes = compileRoutes(config);
  const roots = [...Object.entries(mounts), ["/", root]];
  return createServer((req, res) => {
    const url = new URL(req.url, "http://localhost");
    const host = (req.headers.host ?? "").replace(/:\d+$/, "");
    const { redirect, headers } = applyRoutes(routes, { pathname: url.pathname, host });
    for (const [key, value] of Object.entries(headers)) res.setHeader(key, value);
    if (redirect) {
      res.writeHead(redirect.status, { Location: redirect.location }).end();
      return;
    }
    const file = resolveFile(roots, url.pathname);
    const status = file ? 200 : 404;
    const body = file ?? join(root, "404.html");
    if (!res.hasHeader("Content-Type")) {
      res.setHeader("Content-Type", TYPES[extname(body)] ?? "application/octet-stream");
    }
    if (!res.hasHeader("Cache-Control")) res.setHeader("Cache-Control", DEFAULT_CACHE_CONTROL);
    res.writeHead(status).end(req.method === "HEAD" ? undefined : readFileSync(body));
  });
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const port = Number(process.argv[2] ?? 4322);
  serve().listen(port, "127.0.0.1", () => console.log(`serving dist/ with vercel.json headers at http://127.0.0.1:${port}/`));
}
