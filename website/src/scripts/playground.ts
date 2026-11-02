import { defaultKeymap, history as editHistory, historyKeymap } from "@codemirror/commands";
import { json } from "@codemirror/lang-json";
import { bracketMatching, HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { EditorState, StateEffect, StateField, type Extension } from "@codemirror/state";
import { Decoration, EditorView, keymap, lineNumbers, type DecorationSet } from "@codemirror/view";
import { tags } from "@lezer/highlight";
import { FiscalCalendar, FiscalclockError, validate, type ErrorObject, type Location, type Period } from "fiscalclock";
import type { Example } from "../lib/examples";
import { formatJson } from "../lib/json-format";
import { decodeState, encodeState, type PlaygroundState } from "../lib/playground-state";
import { renderYearStrip } from "../lib/year-strip";
import { rangeForPath, type Range } from "./json-path";

/** Where an error's `path` points: into the definition, or at one of the inputs beside it. */
type ErrorSource = "definition" | "date" | "year";
interface ShownError extends ErrorObject {
  source: ErrorSource;
}

const $ = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
const root = document.querySelector<HTMLElement>(".playground")!;
const examples: Example[] = JSON.parse($("pg-examples").textContent!);
const exampleText = (e: Example) => formatJson(e.definition, 60);
const fallback: PlaygroundState = {
  definition: exampleText(examples[0]!),
  date: root.dataset.exampleDate!,
  year: Number(root.dataset.exampleYear),
};

const exampleSelect = $<HTMLSelectElement>("pg-example");
const exampleSource = $("pg-example-source");
const dateInput = $<HTMLInputElement>("pg-date");
const yearInput = $<HTMLInputElement>("pg-year");
const notice = $("pg-notice");
const errorsBox = $("pg-errors");
const locationBox = $("pg-location");
const stripBox = $("pg-strip");
const stripHeading = $("pg-strip-heading");


const setErrorRanges = StateEffect.define<Range[]>();
const errorMarks = StateField.define<DecorationSet>({
  create: () => Decoration.none,
  update(marks, tr) {
    for (const e of tr.effects) {
      if (e.is(setErrorRanges)) {
        return Decoration.set(e.value.filter((r) => r.to > r.from).map((r) => Decoration.mark({ class: "cm-fc-error" }).range(r.from, r.to)), true);
      }
    }
    return marks.map(tr.changes);
  },
  provide: (field) => EditorView.decorations.from(field),
});

const theme: Extension = [
  EditorView.theme({
    "&": {
      color: "var(--fc-color-code-text)",
      backgroundColor: "var(--fc-color-code-bg)",
      border: "var(--fc-border-width-thin) solid var(--fc-color-border-default)",
      borderRadius: "var(--fc-radius-md)",
      fontSize: "var(--fc-type-code-font-size)",
    },
    "&.cm-focused": { outline: "var(--fc-border-width-thick) solid var(--fc-color-focus-ring)", outlineOffset: "2px" },
    ".cm-scroller": { fontFamily: "var(--fc-type-code-font-family)", lineHeight: "var(--fc-type-code-line-height)", minHeight: "22rem" },
    ".cm-content": { caretColor: "var(--fc-color-text-default)" },
    ".cm-cursor": { borderLeftColor: "var(--fc-color-text-default)" },
    ".cm-gutters": { backgroundColor: "var(--fc-color-code-bg)", color: "var(--fc-color-text-muted)", border: "none" },
    "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, ::selection": { backgroundColor: "var(--fc-color-bg-accent-subtle)" },
    ".cm-matchingBracket": { outline: "var(--fc-border-width-thin) solid var(--fc-color-border-strong)", backgroundColor: "transparent" },
    ".cm-fc-error": {
      textDecoration: "underline wavy var(--fc-color-status-danger)",
      textDecorationSkipInk: "none",
      textUnderlineOffset: "3px",
    },
  }),
  syntaxHighlighting(
    HighlightStyle.define([
      { tag: tags.propertyName, color: "var(--fc-color-code-keyword)" },
      { tag: tags.string, color: "var(--fc-color-code-string)" },
      { tag: [tags.number, tags.bool, tags.null], color: "var(--fc-color-code-text)" },
    ]),
  ),
];

const editor = new EditorView({
  parent: $("pg-editor"),
  state: EditorState.create({
    doc: "",
    extensions: [
      lineNumbers(),
      EditorView.lineWrapping,
      editHistory(),
      bracketMatching(),
      json(),
      errorMarks,
      theme,
      // No indentWithTab: Tab and Shift+Tab move focus in and out of the editor.
      keymap.of([...defaultKeymap, ...historyKeymap]),
      EditorView.contentAttributes.of({ "aria-labelledby": "pg-definition-heading", "aria-describedby": "pg-editor-hint" }),
      EditorView.updateListener.of((u) => {
        if (u.docChanged) schedule();
      }),
    ],
  }),
});

function setDefinition(text: string): void {
  editor.dispatch({ changes: { from: 0, to: editor.state.doc.length, insert: text } });
}


function readState(): PlaygroundState {
  return { definition: editor.state.doc.toString(), date: dateInput.value.trim(), year: Number(yearInput.value) };
}

function applyState(state: PlaygroundState): void {
  setDefinition(state.definition);
  dateInput.value = state.date;
  yearInput.value = String(state.year);
}

function loadFromUrl(): void {
  notice.textContent = "";
  try {
    // Without a fragment, keep a date or year typed while the page was still loading.
    const typed = { date: dateInput.value.trim() || fallback.date, year: Number(yearInput.value) || fallback.year };
    applyState(decodeState(location.hash) ?? { ...fallback, ...typed });
  } catch {
    notice.textContent = "The playground state in this link could not be read, so the first example is shown instead.";
    applyState(fallback);
  }
  render({ followDate: false });
}

let timer: number | undefined;
function schedule(): void {
  clearTimeout(timer);
  timer = window.setTimeout(() => render({ followDate: false }), 200);
}


function el<K extends keyof HTMLElementTagNameMap>(tag: K, attrs: Record<string, string> = {}, ...kids: (Node | string)[]): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs)) node.setAttribute(k, v);
  node.append(...kids);
  return node;
}

const time = (date: string) => el("time", { datetime: date }, date);

function attempt<T>(source: ErrorSource, errors: ShownError[], run: () => T): T | undefined {
  try {
    return run();
  } catch (e) {
    if (!(e instanceof FiscalclockError)) throw e;
    errors.push(...e.errors.map((err) => ({ ...err, source })));
    return undefined;
  }
}

function render({ followDate }: { followDate: boolean }): void {
  const state = readState();
  history.replaceState(null, "", encodeState(state));
  syncExampleSelect(state.definition);

  const errors: ShownError[] = [];
  let here: Location | undefined;
  let cal: FiscalCalendar | undefined;

  const validation = validate(state.definition);
  if ("errors" in validation) {
    errors.push(...validation.errors.map((err) => ({ ...err, source: "definition" as const })));
  } else {
    cal = new FiscalCalendar(state.definition);
    here = attempt("date", errors, () => cal!.locate(state.date));
    if (followDate && here && here.year.year !== state.year) {
      yearInput.value = String(here.year.year);
      state.year = here.year.year;
      history.replaceState(null, "", encodeState(state));
    }
  }

  showLocation(cal, here, state);
  showStrip(cal, here, state, errors);
  showErrors(errors);
}

function showErrors(errors: ShownError[]): void {
  dateInput.setAttribute("aria-invalid", String(errors.some((e) => e.source === "date")));
  yearInput.setAttribute("aria-invalid", String(errors.some((e) => e.source === "year")));
  const ranges = errors.filter((e) => e.source === "definition").map((e) => rangeForPath(editor.state, e.path));
  editor.dispatch({ effects: setErrorRanges.of(ranges.filter((r): r is Range => r !== null)) });

  if (errors.length === 0) {
    errorsBox.replaceChildren();
    return;
  }
  const items = errors.map((e, i) => {
    const range = e.source === "definition" ? ranges[i] : null;
    const where =
      e.source !== "definition"
        ? `in the ${e.source === "date" ? "date" : "fiscal year"} you entered`
        : e.path === ""
          ? "in the whole definition"
          : null;
    const pathNode = where
      ? el("span", {}, where)
      : range
        ? el("button", { type: "button", class: "pg-path", "data-from": String(range.from), "data-to": String(range.to) }, el("code", {}, e.path))
        : el("code", {}, e.path);
    const item = el("li", {}, el("div", { class: "pg-error-head" }, el("code", { class: "pg-error-code" }, e.code), pathNode), el("p", {}, e.message));
    if (e.hint) item.append(el("p", { class: "pg-hint-text" }, `Hint: ${e.hint}`));
    return item;
  });
  const heading = errors.length === 1 ? "fiscalclock reported 1 error" : `fiscalclock reported ${errors.length} errors`;
  errorsBox.replaceChildren(el("p", { class: "pg-errors-heading" }, heading), el("ul", {}, ...items));
}

errorsBox.addEventListener("click", (event) => {
  const button = (event.target as Element).closest<HTMLButtonElement>("button.pg-path");
  if (!button) return;
  const from = Number(button.dataset.from);
  const to = Number(button.dataset.to);
  editor.dispatch({ selection: { anchor: from, head: to }, scrollIntoView: true });
  editor.focus();
});

function labelOf(cal: FiscalCalendar, period: Period): Node {
  try {
    return document.createTextNode(cal.format(period));
  } catch (e) {
    if (!(e instanceof FiscalclockError)) throw e;
    return el("span", { class: "pg-label-missing" }, `none (${e.code})`);
  }
}

function showLocation(cal: FiscalCalendar | undefined, here: Location | undefined, state: PlaygroundState): void {
  if (!cal || !here) {
    locationBox.replaceChildren(el("p", { class: "pg-hint" }, "Results appear here once the definition and the date are valid."));
    return;
  }
  const rows = (["year", "quarter", "period", "week"] as const).map((level) => {
    const p = here[level];
    return el(
      "tr",
      {},
      // The level and its label share a cell, so the table fits a phone's width.
      el("th", { scope: "row" }, el("span", { class: "pg-level" }, level), " ", el("span", { class: "pg-label" }, labelOf(cal, p))),
      el("td", {}, "[", time(p.start), ", ", time(p.end_exclusive), ")"),
      el("td", {}, time(p.last_day)),
      el("td", {}, p.weeks === null ? `${p.days} days` : `${p.weeks}`),
    );
  });
  const head = el("tr", {}, ...["Level and label", "[start, end_exclusive)", "Last day", "Weeks"].map((h) => el("th", { scope: "col" }, h)));
  locationBox.replaceChildren(
    el("div", { class: "pg-table-scroll", tabindex: "0", role: "group", "aria-label": `Where ${state.date} falls; scrolls sideways` }, el("table", {}, el("caption", { class: "sr-only" }, `Where ${state.date} falls`), el("thead", {}, head), el("tbody", {}, ...rows))),
    el(
      "p",
      { class: "pg-day" },
      `Day ${here.day_of_year} of the year, ${here.day_of_quarter} of the quarter, ${here.day_of_period} of the period and ${here.day_of_week} of the week.`,
    ),
  );
}

function showStrip(cal: FiscalCalendar | undefined, here: Location | undefined, state: PlaygroundState, errors: ShownError[]): void {
  stripHeading.textContent = "The fiscal year, to scale";
  if (!cal) {
    stripBox.replaceChildren();
    return;
  }
  const year = attempt("year", errors, () => cal.period("year", state.year));
  if (!year) {
    stripBox.replaceChildren();
    return;
  }
  const periods = cal.children(year, "period");
  const quarters = cal.children(year, "quarter");
  const label = attempt("year", [], () => cal.format(year)) ?? `Fiscal year ${year.year}`;
  const inYear = here?.year.year === year.year;
  const svg = renderYearStrip(periods, {
    id: "pg-strip-svg",
    title: `${label}: ${year.days} days${year.weeks === null ? "" : `, ${year.weeks} weeks`}`,
    quarters,
    highlight: inYear ? here!.period.index! : undefined,
    marker: inYear ? { date: here!.date, dayOfYear: here!.day_of_year } : undefined,
  });
  stripHeading.textContent = `Fiscal year ${state.year}, to scale`;
  const scroller = el("div", { class: "strip-scroll", tabindex: "0", role: "group", "aria-label": `${label}, drawn to scale; scrolls sideways` });
  scroller.innerHTML = svg;
  const note = inYear
    ? `The outlined period holds ${here!.date}, marked by the vertical line.`
    : here
      ? `${here.date} is in fiscal year ${here.year.year}, so it is not marked here.`
      : "";
  stripBox.replaceChildren(scroller, el("p", { class: "pg-hint" }, note));
  // On a narrow screen, bring the marked date into view.
  const marker = scroller.querySelector<SVGRectElement>("[data-marker]");
  if (marker) scroller.scrollLeft = marker.x.baseVal.value - scroller.clientWidth / 2;
}


function syncExampleSelect(text: string): void {
  const match = examples.find((e) => exampleText(e) === text);
  exampleSelect.value = match?.name ?? "";
  exampleSource.replaceChildren();
  if (match) {
    exampleSource.append(match.description, " ", el("a", { href: match.url }, "Source"), ".");
  }
}

exampleSelect.addEventListener("change", () => {
  const example = examples.find((e) => e.name === exampleSelect.value);
  if (!example) return;
  setDefinition(exampleText(example));
  clearTimeout(timer);
  render({ followDate: true });
});

dateInput.addEventListener("input", () => render({ followDate: true }));
yearInput.addEventListener("input", () => render({ followDate: false }));

$("pg-copy").addEventListener("click", async () => {
  const status = $("pg-copy-status");
  try {
    await navigator.clipboard.writeText(location.href);
    status.textContent = "Link copied.";
  } catch {
    status.textContent = "Copying failed. Copy the address from the address bar instead.";
  }
});

window.addEventListener("hashchange", loadFromUrl);
loadFromUrl();
