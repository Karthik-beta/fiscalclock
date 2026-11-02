import { defineConfig, devices } from "@playwright/test";

const port = 4321;

// Tests run against `astro preview` of dist/, so run `npm run build` first.
export default defineConfig({
  testDir: "tests",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  reporter: "list",
  use: {
    baseURL: `http://127.0.0.1:${port}`,
    ...devices["Desktop Chrome"],
  },
  projects: [
    // tests/unit/ holds node:test unit tests, run by `npm test` before Playwright.
    { name: "checks", testIgnore: [/shots\.spec\.ts/, /[\\/]unit[\\/]/] },
    { name: "shots", testMatch: /shots\.spec\.ts/ },
  ],
  webServer: {
    command: `npx astro preview --host 127.0.0.1 --port ${port}`,
    url: `http://127.0.0.1:${port}/`,
    reuseExistingServer: false,
  },
});
