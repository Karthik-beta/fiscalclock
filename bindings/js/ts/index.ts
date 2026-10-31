// Every computation happens in the Rust core, compiled to WebAssembly. This
// module only converts JavaScript values to the core's JSON API and back, so
// results are identical to the Rust crate and the Python package. The
// definition format and operations are specified at
// https://github.com/Karthik-beta/fiscalclock/tree/main/spec.

import {
  Calendar as WasmCalendar,
  call as wasmCall,
  coreVersion as wasmCoreVersion,
  timestampRejected,
  validate as wasmValidate,
} from "../dist/wasm/fiscalclock_wasm.js";

export type Level = "year" | "quarter" | "period" | "week";

/** A fiscal period: the half-open interval `[start, end_exclusive)`. Dates are ISO `YYYY-MM-DD` strings. */
export interface Period {
  level: Level;
  year: number;
  index: number | null;
  start: string;
  end_exclusive: string;
  last_day: string;
  days: number;
  weeks: number | null;
  has_extra_week: boolean;
}

/** Where a date falls at every level. */
export interface Location {
  date: string;
  year: Period;
  quarter: Period;
  period: Period;
  week: Period;
  day_of_year: number;
  day_of_quarter: number;
  day_of_period: number;
  day_of_week: number;
}

/** A period, or just its coordinates. */
export type PeriodRef = Pick<Period, "level" | "year" | "index">;

/** An ISO `YYYY-MM-DD` string or a `Temporal.PlainDate`. */
export type DateLike = string | { readonly [Symbol.toStringTag]: "Temporal.PlainDate"; toString(): string };

export interface ErrorObject {
  code: string;
  path: string;
  message: string;
  hint: string | null;
  details: unknown;
}

export type ValidateResult = { ok: true; calendar_id: string; canonical: string } | { errors: ErrorObject[] };

/** An error from the core, with its stable `code` and JSON `path`. `errors` holds every error reported. */
export class FiscalclockError extends Error {
  readonly errors: ErrorObject[];
  readonly code: string;
  readonly path: string;
  readonly hint: string | null;
  readonly details: unknown;

  constructor(errors: ErrorObject[]) {
    const first = errors[0];
    let text = `${first.code} at ${first.path || "(whole input)"}: ${first.message}`;
    if (first.hint) text += ` (hint: ${first.hint})`;
    if (errors.length > 1) text += `; and ${errors.length - 1} more`;
    super(text);
    this.name = "FiscalclockError";
    this.errors = errors;
    this.code = first.code;
    this.path = first.path;
    this.hint = first.hint;
    this.details = first.details;
  }
}

/** The version of the Rust core compiled into this package. */
export const coreVersion: () => string = wasmCoreVersion;

const TIMESTAMP_TAGS = new Set(["[object Date]", "[object Temporal.PlainDateTime]", "[object Temporal.ZonedDateTime]", "[object Temporal.Instant]"]);

function fromErrorsJson(text: string): FiscalclockError {
  return new FiscalclockError((JSON.parse(text) as { errors: ErrorObject[] }).errors);
}

function dateArg(value: unknown, name: string): unknown {
  const tag = Object.prototype.toString.call(value);
  // A Date is an instant: its calendar day depends on the time zone.
  if (value instanceof Date || TIMESTAMP_TAGS.has(tag)) throw fromErrorsJson(timestampRejected(name));
  if (tag === "[object Temporal.PlainDate]") return String(value);
  // Strings and other JSON values go to the core, which reports INVALID_DATE
  // or INVALID_ARGUMENT for them, so the error matches the other languages.
  if (value === null || ["string", "number", "boolean"].includes(typeof value)) return value;
  return { unsupported_type: tag.slice(8, -1) };
}

function periodArg(period: PeriodRef): PeriodRef {
  return { level: period.level, year: period.year, index: period.index ?? null };
}

function definitionText(definition: string | object): string {
  return typeof definition === "string" ? definition : JSON.stringify(definition);
}

/** Validates a definition. Returns the result object; does not throw for an invalid definition. */
export function validate(definition: string | object): ValidateResult {
  return JSON.parse(wasmValidate(definitionText(definition))) as ValidateResult;
}

/** Validates definition text and returns the core's JSON unchanged. */
export function validateJson(definition: string): string {
  return wasmValidate(definition);
}

/** Runs one operation on JSON text and returns the core's JSON unchanged. */
export function callJson(definition: string, op: string, args: string): string {
  return wasmCall(definition, op, args);
}

/** A calendar built from a validated definition. Throws `FiscalclockError` if the definition is invalid. */
export class FiscalCalendar {
  readonly #cal: WasmCalendar;

  constructor(definition: string | object) {
    try {
      this.#cal = new WasmCalendar(definitionText(definition));
    } catch (e) {
      throw typeof e === "string" ? fromErrorsJson(e) : e;
    }
  }

  /** The first 16 hex digits of the SHA-256 of the canonical form. */
  get calendarId(): string {
    return this.#cal.calendarId;
  }

  /** The canonical form of the definition. */
  get canonicalJson(): string {
    return this.#cal.canonicalJson;
  }

  /** Runs operation `op` with JSON arguments; returns the core's JSON unchanged. */
  callJson(op: string, args: string): string {
    return this.#cal.callJson(op, args);
  }

  #call<T>(op: string, args: Record<string, unknown>): T {
    const out: unknown = JSON.parse(this.#cal.callJson(op, JSON.stringify(args)));
    if (out !== null && typeof out === "object" && !Array.isArray(out) && "errors" in out) {
      throw new FiscalclockError((out as { errors: ErrorObject[] }).errors);
    }
    return out as T;
  }

  /** The year, quarter, period and week containing `date`. */
  locate(date: DateLike): Location {
    return this.#call("locate", { date: dateArg(date, "date") });
  }

  /** The period with coordinates `(level, year, index)`; omit `index` for a year. */
  period(level: Level, year: number, index?: number | null): Period {
    return this.#call("period", index == null ? { level, year } : { level, year, index });
  }

  /** The period `n` steps away at the same level. */
  shift(period: PeriodRef, n: number): Period {
    return this.#call("shift", { period: periodArg(period), n });
  }

  /** The enclosing period one level up. */
  parent(period: PeriodRef): Period {
    return this.#call("parent", { period: periodArg(period) });
  }

  /** The periods at `level` inside `period`, in order. */
  children(period: PeriodRef, level: Level): Period[] {
    return this.#call("children", { period: periodArg(period), level });
  }

  /** The fiscal years numbered `first` to `last` inclusive. */
  years(first: number, last: number): Period[] {
    return this.#call("years", { from: first, to: last });
  }

  /** The label of `period` under `scheme`, or the default scheme. */
  format(period: PeriodRef, scheme?: string): string {
    const args: Record<string, unknown> = { period: periodArg(period) };
    if (scheme !== undefined) args.scheme = scheme;
    return this.#call<{ label: string }>("format", args).label;
  }

  /** The period a label names. Throws with code `AMBIGUOUS_LABEL` when several match. */
  parse(label: string): Period {
    return this.#call("parse", { label });
  }

  /** One row per day in `[start, endExclusive)`. */
  dimension(start: DateLike, endExclusive: DateLike): Record<string, unknown>[] {
    return this.#call("dimension", { start: dateArg(start, "start"), end_exclusive: dateArg(endExclusive, "end_exclusive") });
  }

  /** The rows of `dimension` as RFC 4180 CSV text. */
  dimensionCsv(start: DateLike, endExclusive: DateLike): string {
    return this.#call("dimension_csv", { start: dateArg(start, "start"), end_exclusive: dateArg(endExclusive, "end_exclusive") });
  }

  /** The rules applied to locate `date`, step by step. */
  explain(date: DateLike): { date: string; calendar_id: string; steps: { level: Level; rule: string; result: string }[] } {
    return this.#call("explain", { date: dateArg(date, "date") });
  }
}
