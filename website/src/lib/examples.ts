/**
 * The example definitions in conformance/vectors/cited/, read at build time.
 * They are documentation examples with their cited sources, not library
 * presets: fiscalclock itself has no named calendars.
 */

export interface VectorFile {
  name: string;
  description: string;
  source: { url: string | null };
  definition: Record<string, unknown>;
  cases: { op: string; args: Record<string, unknown>; expect?: unknown; expect_subset?: Record<string, unknown> }[];
}

export interface Example {
  name: string;
  description: string;
  url: string;
  definition: Record<string, unknown>;
}

const cited = import.meta.glob<VectorFile>("../../../conformance/vectors/cited/*.json", { eager: true, import: "default" });

/** Every cited vector, NRF first because the landing page draws it. */
export const examples: Example[] = Object.values(cited)
  .map(({ name, description, source, definition }) => ({ name, description, url: source.url ?? "", definition }))
  .sort((a, b) => Number(b.name.startsWith("nrf-")) - Number(a.name.startsWith("nrf-")) || a.name.localeCompare(b.name));

export const nrf: VectorFile = Object.values(cited).find((v) => v.name === "nrf-53-week-years")!;
