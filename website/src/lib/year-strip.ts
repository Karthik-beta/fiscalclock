/**
 * Draws one fiscal year as a to-scale strip: width is proportional to days,
 * every period is a half-open interval [start, end_exclusive), and the extra
 * week is hatched and labelled. It draws only what fiscalclock returns: no
 * date in the picture is computed here.
 *
 * The same function renders the landing page at build time and the
 * playground in the browser, so it returns a plain string and touches no DOM.
 */

/** The keys of the core's Period JSON (spec/api-v1.md, "Period") that the strip reads. */
export interface StripPeriod {
  index: number | null;
  start: string;
  end_exclusive: string;
  days: number;
  weeks: number | null;
  has_extra_week: boolean;
}

export interface StripOptions {
  /** Unique within the page; prefixes the SVG's internal ids. */
  id: string;
  /** Accessible name, for example "FY2023: 371 days, 53 weeks". */
  title: string;
  /** The year's quarters, if the strip should mark them. */
  quarters?: StripPeriod[];
  /** Index of a period to outline, such as the one a located date falls in. */
  highlight?: number;
  /** A caller-supplied date to mark, with its 1-based day of the year from `locate`. */
  marker?: { date: string; dayOfYear: number };
}

const PX_PER_DAY = 2.4;
// Wide enough for half an ISO date, so every boundary label can sit centred on its line.
const PAD_X = 36;
const LABEL = 12;
const SMALL = 11;

const Y_QUARTER = 14;
const Y_BAND = 24;
const BAND_H = 48;
const Y_AXIS = Y_BAND + BAND_H + 12;
const Y_DATES = [Y_AXIS + 22, Y_AXIS + 38];
const Y_NOTES = Y_AXIS + 56;
const Y_MARKER = Y_AXIS + 76;

const color = (role: string) => `var(--fc-color-diagram-${role})`;

function esc(text: string): string {
  return text.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]!);
}

function rect(x: number, y: number, w: number, h: number, style: string, attrs = ""): string {
  return `<rect${attrs} x="${x.toFixed(2)}" y="${y}" width="${w.toFixed(2)}" height="${h}" style="${style}"/>`;
}

function text(x: number, y: number, body: string, style: string, anchor = "start"): string {
  return `<text x="${x.toFixed(2)}" y="${y}" text-anchor="${anchor}" style="${style}">${esc(body)}</text>`;
}

const interval = (p: StripPeriod) => `[${p.start}, ${p.end_exclusive})`;
const weeksText = (p: StripPeriod) => (p.weeks === null ? `${p.days} days` : `${p.weeks} weeks`);

export function describeYear(periods: StripPeriod[], quarters: StripPeriod[] = []): string {
  const first = periods[0]!;
  const last = periods[periods.length - 1]!;
  const lines = [`Year [${first.start}, ${last.end_exclusive}).`];
  for (const q of quarters) lines.push(`Q${q.index} ${interval(q)}, ${weeksText(q)}.`);
  for (const p of periods) {
    const extra = p.has_extra_week ? ", including the extra week as its last week" : "";
    lines.push(`P${p.index} ${interval(p)}, ${weeksText(p)}${extra}.`);
  }
  return lines.join(" ");
}

export function renderYearStrip(periods: StripPeriod[], opts: StripOptions): string {
  if (periods.length === 0) throw new Error("renderYearStrip needs at least one period");
  const totalDays = periods.reduce((sum, p) => sum + p.days, 0);
  const width = PAD_X * 2 + totalDays * PX_PER_DAY;
  const height = (opts.marker ? Y_MARKER : Y_NOTES) + 8;
  const xOf = (dayOffset: number) => PAD_X + dayOffset * PX_PER_DAY;
  const offsets: number[] = [];
  periods.reduce((acc, p) => (offsets.push(acc), acc + p.days), 0);
  const offsetOf = new Map(periods.map((p, i) => [p.start, offsets[i]!]));

  const ink = `fill:${color("ink")}`;
  const muted = `fill:${color("ink-muted")}`;
  const semibold = "font-weight:600";
  const hatch = `${opts.id}-hatch`;
  const out: string[] = [];

  out.push(
    `<defs><pattern id="${hatch}" width="6" height="6" patternUnits="userSpaceOnUse" patternTransform="rotate(45)">` +
      `<rect width="2" height="6" style="fill:${color("extra-week")}"/></pattern></defs>`,
    rect(0, 0, width, height, `fill:${color("surface")}`),
  );

  periods.forEach((p, i) => {
    const x = xOf(offsets[i]!);
    const w = p.days * PX_PER_DAY;
    if (i % 2 === 0) out.push(rect(x, Y_BAND, w, BAND_H, `fill:${color("period-fill")}`));
    if (p.weeks !== null) {
      for (let d = 7; d < p.days; d += 7) out.push(rect(xOf(offsets[i]! + d) - 0.5, Y_BAND + BAND_H - 8, 1, 8, muted));
    }
    // The extra week is the last week of its period (spec/definition-v1.md).
    const labelSpan = p.has_extra_week && p.weeks !== null ? w - 7 * PX_PER_DAY : w;
    if (p.has_extra_week && p.weeks !== null) {
      const ex = x + w - 7 * PX_PER_DAY;
      out.push(
        rect(ex, Y_BAND, 7 * PX_PER_DAY, BAND_H, `fill:url(#${hatch})`),
        rect(ex + 1, Y_BAND + 1, 7 * PX_PER_DAY - 2, BAND_H - 2, `fill:none;stroke:${color("extra-week")};stroke-width:2`),
      );
    }
    out.push(
      text(x + labelSpan / 2, Y_BAND + 21, `P${p.index}`, `${ink};${semibold};font-size:${LABEL}px`, "middle"),
      text(x + labelSpan / 2, Y_BAND + 36, p.weeks === null ? `${p.days} d` : `${p.weeks} wk`, `${muted};font-size:${SMALL}px`, "middle"),
    );
    if (i > 0) out.push(rect(x - 0.75, Y_BAND, 1.5, BAND_H, `fill:${color("rule")}`));
  });

  for (const q of opts.quarters ?? []) {
    const offset = offsetOf.get(q.start);
    if (offset === undefined) continue;
    const x = xOf(offset);
    if (offset > 0) out.push(rect(x - 1.5, Y_QUARTER - 10, 3, Y_BAND + BAND_H - Y_QUARTER + 10, `fill:${color("ink")}`));
    out.push(
      `<text x="${(x + 6).toFixed(2)}" y="${Y_QUARTER}" style="font-size:${LABEL}px">` +
        `<tspan style="${ink};${semibold}">Q${q.index}</tspan>` +
        `<tspan dx="5" style="${muted}">${esc(weeksText(q))}</tspan></text>`,
    );
  }

  const extra = periods.find((p) => p.has_extra_week && p.weeks !== null);
  if (extra) {
    const totalWeeks = periods.reduce((sum, p) => sum + (p.weeks ?? 0), 0);
    out.push(text(width - PAD_X, Y_QUARTER, `extra week ${totalWeeks}`, `fill:${color("extra-week")};${semibold};font-size:${LABEL}px`, "end"));
  }

  if (opts.highlight !== undefined) {
    const i = periods.findIndex((p) => p.index === opts.highlight);
    if (i >= 0) {
      const x = xOf(offsets[i]!);
      out.push(rect(x + 1.5, Y_BAND + 1.5, periods[i]!.days * PX_PER_DAY - 3, BAND_H - 3, `fill:none;stroke:${color("reference-date")};stroke-width:3`));
    }
  }

  // Boundaries: one line per boundary, closed marker at the start, open marker at end_exclusive.
  const last = periods[periods.length - 1]!;
  out.push(rect(PAD_X, Y_AXIS - 1, totalDays * PX_PER_DAY, 2, `fill:${color("ink")}`));
  const bounds = [...periods.map((p, i) => ({ date: p.start, x: xOf(offsets[i]!) })), { date: last.end_exclusive, x: xOf(totalDays) }];
  bounds.forEach((b, i) => {
    const row = i % 2;
    out.push(
      rect(b.x - 0.5, Y_AXIS, 1, Y_DATES[row]! - Y_AXIS - LABEL, muted),
      text(b.x, Y_DATES[row]!, b.date, `${ink};font-size:${LABEL}px`, "middle"),
    );
  });
  out.push(
    `<circle cx="${PAD_X}" cy="${Y_AXIS}" r="5" style="fill:${color("ink")}"/>`,
    `<circle cx="${xOf(totalDays).toFixed(2)}" cy="${Y_AXIS}" r="4.5" style="fill:${color("surface")};stroke:${color("open-end")};stroke-width:2.5"/>`,
    text(PAD_X - 30, Y_NOTES, "start, included", `${muted};font-size:${SMALL}px`),
    text(xOf(totalDays) + 30, Y_NOTES, "end_exclusive, excluded", `${muted};font-size:${SMALL}px`, "end"),
  );

  if (opts.marker) {
    const x = xOf(opts.marker.dayOfYear - 1 + 0.5);
    const anchor = x < width * 0.15 ? "start" : x > width * 0.85 ? "end" : "middle";
    out.push(
      rect(x - 1, Y_BAND - 4, 2, Y_AXIS - Y_BAND + 10, `fill:${color("reference-date")}`, " data-marker"),
      text(x, Y_MARKER, opts.marker.date, `fill:${color("reference-date")};${semibold};font-size:${LABEL}px`, anchor),
    );
  }

  const title = `${opts.id}-title`;
  const desc = `${opts.id}-desc`;
  return (
    `<svg xmlns="http://www.w3.org/2000/svg" role="img" aria-labelledby="${title}" aria-describedby="${desc}" ` +
    `width="${width.toFixed(0)}" height="${height}" viewBox="0 0 ${width.toFixed(2)} ${height}" class="year-strip" ` +
    `style="font-family:var(--fc-type-body-font-family);font-variant-numeric:tabular-nums lining-nums">` +
    `<title id="${title}">${esc(opts.title)}</title>` +
    `<desc id="${desc}">${esc(describeYear(periods, opts.quarters))}</desc>` +
    out.join("") +
    `</svg>`
  );
}
