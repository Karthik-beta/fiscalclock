/**
 * The landing page's hero copy and install commands. Components/Hero.astro
 * draws them as HTML and agents/mdx-to-markdown.ts writes the same text into
 * the page's Markdown copy, so neither keeps its own.
 */
export const HERO_SUMMARY =
  "Describe your fiscal calendar as JSON. fiscalclock tells you which fiscal year, quarter, period and week any date falls in, with the bounds and labels of each. One Rust core gives the same answer in Python, JavaScript and Rust.";

/** In tab order: Python first (DD-034). */
export const INSTALL_COMMANDS = [
  { label: "Python", command: "pip install fiscalclock" },
  { label: "JavaScript", command: "npm install fiscalclock" },
  { label: "Rust", command: "cargo add fiscalclock" },
];
