/**
 * The landing page's date lookup: where one date falls under one example
 * definition. The same functions run at build time, to draw the first answer
 * into the HTML, and in the browser when a visitor asks another question, so
 * the page shows an answer before any WebAssembly loads. The module is passed
 * in, so importing this file never loads it.
 */
import type { ErrorObject, FiscalCalendar, Period } from "fiscalclock";

type Fiscalclock = Pick<typeof import("fiscalclock"), "FiscalCalendar" | "FiscalclockError">;

const LEVELS = ["year", "quarter", "period", "week"] as const;
type Level = (typeof LEVELS)[number];

interface Row {
  level: Level;
  label: string;
  start: string;
  end_exclusive: string;
}

/** One period of the located year, drawn as a segment as wide as its days. */
interface Segment {
  days: number;
  weeks: number | null;
  here: boolean;
  /** The 0-based week of the period that holds the date, in a week-based period. */
  currentWeek: number | null;
}

export type Lookup =
  | { ok: true; date: string; headline: string; rows: Row[]; segments: Segment[] }
  | { ok: false; date: string; errors: ErrorObject[] };

/** A definition without a label template for a level still has the level's index. */
function labelOf(fc: Fiscalclock, cal: FiscalCalendar, p: Period): string {
  try {
    return cal.format(p);
  } catch (e) {
    if (!(e instanceof fc.FiscalclockError)) throw e;
    return `${p.level[0]!.toUpperCase()}${p.level.slice(1)} ${p.index ?? p.year}`;
  }
}

export function lookup(fc: Fiscalclock, definition: Record<string, unknown>, date: string): Lookup {
  try {
    const cal = new fc.FiscalCalendar(definition);
    const here = cal.locate(date);
    const year = cal.period("year", here.year.year);
    const segments = cal.children(year, "period").map((p) => {
      const isHere = p.index === here.period.index;
      return { days: p.days, weeks: p.weeks, here: isHere, currentWeek: isHere && p.weeks !== null ? Math.floor((here.day_of_period - 1) / 7) : null };
    });
    const rows = LEVELS.map((level) => ({ level, label: labelOf(fc, cal, here[level]), start: here[level].start, end_exclusive: here[level].end_exclusive }));
    return { ok: true, date, headline: rows[1]!.label, rows, segments };
  } catch (e) {
    if (!(e instanceof fc.FiscalclockError)) throw e;
    return { ok: false, date, errors: e.errors };
  }
}

const esc = (text: string) => text.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]!);
const time = (d: string) => `<time datetime="${esc(d)}">${esc(d)}</time>`;

function ribbon(segments: Segment[]): string {
  const cells = segments.map((s) => {
    const weeks = s.weeks ?? 0;
    const marks = Array.from({ length: weeks }, (_, i) => `<span${i === s.currentWeek ? ' class="is-here"' : ""}></span>`).join("");
    return `<span class="lookup-segment${s.here ? " is-here" : ""}" style="flex-grow:${s.days}">${marks}</span>`;
  });
  return `<div class="lookup-ribbon" aria-hidden="true">${cells.join("")}</div>`;
}

/** What the page says when the WebAssembly module cannot be loaded, so it never keeps a stale answer. */
export const LOAD_FAILED = "fiscalclock could not be loaded";
export const LOAD_FAILED_HINT = "Reload the page to try again.";

export function renderLoadFailure(): string {
  return `<p class="lookup-headline is-error">${LOAD_FAILED}</p><p>${LOAD_FAILED_HINT}</p>`;
}

/** The answer as HTML. Every value in it is fiscalclock's own output, escaped. */
export function renderLookup(result: Lookup): string {
  if (!result.ok) {
    const items = result.errors.map((e) => `<li><code>${esc(e.code)}</code> ${esc(e.message)}${e.hint ? ` Hint: ${esc(e.hint)}` : ""}</li>`);
    return `<p class="lookup-headline is-error">No answer for ${esc(result.date)}</p><ul class="lookup-errors">${items.join("")}</ul>`;
  }
  const rows = result.rows.map(
    (r) => `<tr><th scope="row">${r.level}</th><td class="lookup-label">${esc(r.label)}</td><td>[${time(r.start)}, ${time(r.end_exclusive)})</td></tr>`,
  );
  return (
    `<p class="lookup-headline">${esc(result.headline)}</p>` +
    `<table class="lookup-table"><caption class="sr-only">Where ${esc(result.date)} falls, as [start, end_exclusive)</caption><tbody>${rows.join("")}</tbody></table>` +
    ribbon(result.segments)
  );
}
