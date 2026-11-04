import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import type { Server } from "node:http";
import type { AddressInfo } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { expect, test, type Page } from "@playwright/test";
import { serve } from "../scripts/serve-with-headers.mjs";
import { builtPages } from "./site";

// Loads the built site from a server that applies vercel.json's headers, and
// fails on any Content-Security-Policy violation or console error.

const FIXTURE = "/__csp-fixture/";

// (module (func (export "add") (param i32 i32) (result i32) local.get 0 local.get 1 i32.add))
const ADD_WASM = Uint8Array.from([
  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02,
  0x01, 0x00, 0x07, 0x07, 0x01, 0x03, 0x61, 0x64, 0x64, 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20,
  0x01, 0x6a, 0x0b,
]);

const FIXTURE_FILES: Record<string, string | Uint8Array> = {
  "add.wasm": ADD_WASM,
  "add.bin": ADD_WASM,
  "wasm.html": `<!doctype html><title>wasm</title><script type="module" src="${FIXTURE}wasm.js"></script>`,
  "wasm.js": `try {
  const { instance } = await WebAssembly.instantiateStreaming(fetch("${FIXTURE}" + location.hash.slice(1)));
  document.documentElement.dataset.sum = String(instance.exports.add(2, 3));
} catch (error) {
  document.documentElement.dataset.error = error.name;
}`,
  "inline.html": `<!doctype html><title>inline</title><script>document.title = "ran";</script>`,
};

let server: Server;
let origin: string;
let fixtures: string;

test.beforeAll(async () => {
  fixtures = mkdtempSync(join(tmpdir(), "csp-fixture-"));
  for (const [name, content] of Object.entries(FIXTURE_FILES)) writeFileSync(join(fixtures, name), content);
  server = serve({ mounts: { [FIXTURE]: fixtures } });
  await new Promise<void>((resolve) => server.listen(0, "127.0.0.1", resolve));
  origin = `http://127.0.0.1:${(server.address() as AddressInfo).port}`;
});

test.afterAll(async () => {
  await new Promise((resolve) => server.close(resolve));
  rmSync(fixtures, { recursive: true, force: true });
});

/** Starts recording CSP violations and errors; call the result to read them. */
async function watch(page: Page): Promise<() => Promise<string[]>> {
  const problems: string[] = [];
  page.on("console", (m) => m.type() === "error" && problems.push(`console: ${m.text()}`));
  page.on("pageerror", (e) => problems.push(`page error: ${e.message}`));
  page.on("worker", (w) => w.on("console", (m) => m.type() === "error" && problems.push(`worker: ${m.text()}`)));
  await page.addInitScript(() => {
    const seen: string[] = ((window as unknown as { __csp: string[] }).__csp = []);
    document.addEventListener("securitypolicyviolation", (e) => seen.push(`${e.effectiveDirective} ${e.blockedURI}`));
  });
  return async () => {
    const violations = await page.evaluate(() => (window as unknown as { __csp: string[] }).__csp);
    return [...violations.map((v) => `violation: ${v}`), ...problems];
  };
}

for (const path of builtPages()) {
  test(`${path} loads under the Content-Security-Policy with no violations`, async ({ page }) => {
    const problems = await watch(page);
    const response = await page.goto(origin + path, { waitUntil: "networkidle" });
    expect(response?.headers()["content-security-policy"]).toContain("frame-ancestors 'none'");
    expect(await problems()).toEqual([]);
  });
}

test("Pagefind search runs under the Content-Security-Policy", async ({ page }) => {
  const problems = await watch(page);
  await page.goto(origin + "/");
  await page.locator("button[data-open-modal]").click();
  await page.locator(".pagefind-ui__search-input").fill("fiscal");
  await expect(page.locator(".pagefind-ui__result").first()).toBeVisible();
  await page.waitForLoadState("networkidle");
  expect(await problems()).toEqual([]);
});

test("WebAssembly served as application/wasm compiles by streaming under the policy", async ({ page }) => {
  const problems = await watch(page);
  await page.goto(`${origin}${FIXTURE}wasm.html#add.wasm`);
  await expect(page.locator("html")).toHaveAttribute("data-sum", "5");
  expect(await problems()).toEqual([]);
});

test("control: without the application/wasm type, streaming compilation fails", async ({ page }) => {
  await page.goto(`${origin}${FIXTURE}wasm.html#add.bin`);
  await expect(page.locator("html")).toHaveAttribute("data-error", "TypeError");
});

test("control: an inline script that is not in script-src is blocked and reported", async ({ page }) => {
  const problems = await watch(page);
  await page.goto(`${origin}${FIXTURE}inline.html`);
  await expect(page).toHaveTitle("inline");
  expect(await problems()).toContainEqual(expect.stringMatching(/^violation: script-src-elem inline$/));
});
