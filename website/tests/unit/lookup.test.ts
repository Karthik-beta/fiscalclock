import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import * as fiscalclock from "fiscalclock";
import { LOAD_FAILED, lookup, renderLoadFailure, renderLookup, type Lookup } from "../../src/lib/lookup.ts";

const cited = (file: string) =>
  JSON.parse(readFileSync(new URL(`../../../conformance/vectors/cited/${file}`, import.meta.url), "utf8")).definition as Record<string, unknown>;

// Reliance's annual report gives a year from 1 April and its quarter names, and no names for periods or weeks.
const reliance = cited("reliance-annual-report-2024-25.json");

function answered(result: Lookup) {
  assert.equal(result.ok, true, JSON.stringify(result));
  return result as Extract<Lookup, { ok: true }>;
}

test("a fixed-date year is answered with its bounds, and its periods have no weeks to mark", () => {
  const result = answered(lookup(fiscalclock, reliance, "2024-08-15"));
  const [year, quarter] = result.rows;
  // Financial year 2024-25 runs from 1 April 2024 to 31 March 2025 (Reliance Integrated Annual Report 2024-25, p. 58 and p. 114).
  assert.deepEqual([year!.start, year!.end_exclusive], ["2024-04-01", "2025-04-01"]);
  assert.equal(quarter!.label, "Q2 FY 2024-25");
  assert.equal(result.headline, "Q2 FY 2024-25");
  assert.equal(result.segments.length, 12);
  assert.equal(result.segments.filter((s) => s.here).length, 1);
  assert.ok(result.segments.every((s) => s.weeks === null && s.currentWeek === null));
});

test("a level without a label template is named by its index instead of failing", () => {
  const result = answered(lookup(fiscalclock, reliance, "2024-08-15"));
  const period = result.rows.find((r) => r.level === "period")!;
  // Apr is period 1, so Aug is period 5.
  assert.equal(period.label, "Period 5");
  assert.deepEqual([period.start, period.end_exclusive], ["2024-08-01", "2024-09-01"]);
  assert.match(result.rows.find((r) => r.level === "week")!.label, /^Week /);
});

test("an invalid date is reported with the library's error codes, not thrown", () => {
  const result = lookup(fiscalclock, reliance, "2024-02-30");
  assert.equal(result.ok, false);
  assert.ok(!result.ok && result.errors.some((e) => e.code === "INVALID_DATE"));
});

test("an invalid definition is reported, not thrown", () => {
  const result = lookup(fiscalclock, {}, "2024-02-03");
  assert.equal(result.ok, false);
  assert.ok(!result.ok && result.errors.length > 0);
});

test("an error that is not the library's is not swallowed", () => {
  const broken = {
    FiscalclockError: fiscalclock.FiscalclockError,
    FiscalCalendar: class {
      constructor() {
        throw new TypeError("not a library error");
      }
    },
  } as unknown as Parameters<typeof lookup>[0];
  assert.throws(() => lookup(broken, reliance, "2024-08-15"), TypeError);
});

test("error text is escaped before it goes into the page", () => {
  const html = renderLookup({
    ok: false,
    date: '<img src=x onerror="alert(1)">',
    errors: [{ code: "A<B", path: "", message: 'says "<b>&</b>"', hint: "<i>try again</i>", details: null }],
  });
  assert.ok(!/<(img|b|i)[ >]/.test(html), html);
  assert.ok(html.includes("No answer for &lt;img src=x onerror=&quot;alert(1)&quot;&gt;"), html);
  assert.ok(html.includes("<code>A&lt;B</code>"), html);
  assert.ok(html.includes("says &quot;&lt;b&gt;&amp;&lt;/b&gt;&quot;"), html);
  assert.ok(html.includes("Hint: &lt;i&gt;try again&lt;/i&gt;"), html);
});

test("an answer's labels are escaped", () => {
  const html = renderLookup({
    ok: true,
    date: "2024-02-03",
    headline: "<script>",
    rows: [{ level: "year", label: "A&B", start: "2024-01-01", end_exclusive: "2025-01-01" }],
    segments: [],
  });
  assert.ok(html.includes('<p class="lookup-headline">&lt;script&gt;</p>'), html);
  assert.ok(html.includes("A&amp;B"), html);
});

test("the load-failure message names the problem and the way out", () => {
  const html = renderLoadFailure();
  assert.ok(html.includes(LOAD_FAILED), html);
  assert.ok(html.includes("Reload the page"), html);
});
