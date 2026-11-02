import { defineConfig, devices } from "@playwright/test";

// SITE_PORT lets two checkouts run their tests at the same time.
const port = Number(process.env.SITE_PORT ?? 4321);

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
    { name: "checks", testIgnore: /shots\.spec\.ts/ },
    { name: "shots", testMatch: /shots\.spec\.ts/ },
  ],
  webServer: {
    command: `npx astro preview --host 127.0.0.1 --port ${port}`,
    url: `http://127.0.0.1:${port}/`,
    reuseExistingServer: false,
  },
});
