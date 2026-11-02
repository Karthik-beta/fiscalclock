import assert from "node:assert/strict";
import { test } from "node:test";
import { mdxToMarkdown, type MdxPage } from "../../src/agents/mdx-to-markdown.ts";

const FILES: Record<string, string> = {
  "./quick_start.py": 'print("FY2023")\n',
  "./quick_start.js": 'console.log("FY2023");\n',
};

const MODULES: Record<string, Record<string, unknown>> = {
  "../../lib/landing": { definitionText: '{"spec_version": 1}', output: "Q4 FY2023\n", sourceUrl: "https://nrf.com/" },
};

function page(body: string): MdxPage {
  return {
    id: "start-here/quick-start",
    title: "Quick start",
    description: "Compute a fiscal year.",
    body,
    format: "mdx",
    url: "https://fiscalclock.karthik.nz/start-here/quick-start/",
    readRaw: (specifier) => {
      const text = FILES[specifier];
      if (text === undefined) throw new Error(`no file ${specifier}`);
      return text;
    },
    readModule: async (specifier) => {
      const module = MODULES[specifier];
      if (module === undefined) throw new Error(`no module ${specifier}`);
      return module;
    },
  };
}

const TABS = `import { Code, Tabs, TabItem } from "@astrojs/starlight/components";
import py from "./quick_start.py?raw";
import js from "./quick_start.js?raw";

<Tabs syncKey="lang">
  <TabItem label="Python">
    <Code code={py} lang="python" title="quick_start.py" />
  </TabItem>
  <TabItem label="JavaScript">
    <Code code={js} lang="js" />
  </TabItem>
</Tabs>
`;

test("the title and description open the page", async () => {
  assert.match(await mdxToMarkdown(page("Body.")), /^# Quick start\n\nCompute a fiscal year\.\n\nBody\.\n$/);
});

test("code imported with ?raw becomes a fenced block under its tab label and title", async () => {
  const markdown = await mdxToMarkdown(page(TABS));
  assert.ok(markdown.includes('**Python**\n\n*quick_start.py*\n\n```python\nprint("FY2023")\n```'), markdown);
  assert.ok(markdown.includes('**JavaScript**\n\n```js\nconsole.log("FY2023");\n```'), markdown);
  assert.doesNotMatch(markdown, /<Tabs|<TabItem|<Code|^import /m);
});

test("code and links can come from values a module exports", async () => {
  const markdown = await mdxToMarkdown(
    page(`import * as fy from "../../lib/landing";

<Code code={fy.definitionText} lang="json" title="retail-4-5-4.json" />

<RawOutput text={fy.output} label="Output" />

<p class="summary">See{" "}<a href={fy.sourceUrl}>NRF</a>.</p>
`),
  );
  assert.ok(markdown.includes('*retail-4-5-4.json*\n\n```json\n{"spec_version": 1}\n```'), markdown);
  assert.ok(markdown.includes("*Output*\n\n```text\nQ4 FY2023\n```"), markdown);
  assert.ok(markdown.includes("See [NRF](https://nrf.com/)."), markdown);
});

test("a plain Markdown page is read as Markdown, without its HTML comments", async () => {
  const markdown = await mdxToMarkdown({ ...page("<!-- generated -->\n\nYears with {braces} and a < b.\n"), format: "md" });
  assert.ok(markdown.includes("Years with {braces} and a < b."), markdown);
  assert.doesNotMatch(markdown, /generated/);
});

test("an aside becomes a blockquote that starts with its type and title", async () => {
  const markdown = await mdxToMarkdown(page('<Aside type="caution" title="Extra week">Week 53 goes in period 12.</Aside>\n'));
  assert.ok(markdown.includes("> **Caution: Extra week**\n>\n> Week 53 goes in period 12."), markdown);
});

test("relative links become absolute URLs on the site", async () => {
  const markdown = await mdxToMarkdown(
    page('See [the spec](../../reference/spec/) and [errors](#errors).\n\n<LinkCard title="API" href="/reference/api/" />\n'),
  );
  assert.ok(markdown.includes("[the spec](https://fiscalclock.karthik.nz/reference/spec/)"), markdown);
  assert.ok(markdown.includes("[errors](https://fiscalclock.karthik.nz/start-here/quick-start/#errors)"), markdown);
  assert.ok(markdown.includes("[API](https://fiscalclock.karthik.nz/reference/api/)"), markdown);
});

test("HTML elements keep their text", async () => {
  assert.match(await mdxToMarkdown(page('Ends on <time datetime="2024-02-03">3 Feb 2024</time>.\n')), /Ends on 3 Feb 2024\./);
});

test("a component with no conversion fails and names itself and the page", async () => {
  await assert.rejects(
    mdxToMarkdown(page("<Steps>\n\n1. Install.\n\n</Steps>\n")),
    /start-here\/quick-start: no Markdown conversion for the component <Steps>/,
  );
});

test("an expression that is not an imported value fails", async () => {
  await assert.rejects(mdxToMarkdown(page('<Code code={example} lang="python" />\n')), /the expression \{example\} is not/);
  await assert.rejects(mdxToMarkdown(page("{1 + 1}\n")), /the expression \{1 \+ 1\} is not/);
});
