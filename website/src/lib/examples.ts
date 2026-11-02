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

// What a definition's year is, from `year.type` (spec/definition-v1.md).
const YEAR_KINDS: Record<string, string> = { week_based: "52/53-week year", fixed_date: "fixed-date year" };

/**
 * A label short enough for a closed select, such as "Costco, 52/53-week year".
 * The organisation is the first word of the vector's name, spelt as its
 * description spells it; the build fails if the description does not name it.
 */
export function shortLabel({ name, description, definition }: Example): string {
  const word = name.split("-")[0]!;
  const organisation = description.match(new RegExp(`\\b${word}\\b`, "i"))?.[0];
  const kind = YEAR_KINDS[(definition.year as { type: string }).type];
  if (!organisation || !kind) throw new Error(`No short label for the example ${name}: name its organisation in the description and use a known year type.`);
  return `${organisation}, ${kind}`;
}

export const nrf: VectorFile = Object.values(cited).find((v) => v.name === "nrf-53-week-years")!;
