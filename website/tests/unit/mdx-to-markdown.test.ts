import assert from "node:assert/strict";
import { test } from "node:test";
import { mdxToMarkdown, type MdxPage } from "../../src/agents/mdx-to-markdown.ts";

const FILES: Record<string, string> = {
  "./quick_start.py": 'print("FY2023")\n',
  "./quick_start.js": 'console.log("FY2023");\n',
};

function page(body: string): MdxPage {
  return {
    id: "start-here/quick-start",
    title: "Quick start",
    description: "Compute a fiscal year.",
    body,
    url: "https://fiscalclock.karthik.nz/start-here/quick-start/",
    readRaw: (specifier) => {
      const text = FILES[specifier];
      if (text === undefined) throw new Error(`no file ${specifier}`);
      return text;
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

test("the title and description open the page", () => {
  assert.match(mdxToMarkdown(page("Body.")), /^# Quick start\n\nCompute a fiscal year\.\n\nBody\.\n$/);
});

test("code imported with ?raw becomes a fenced block under its tab label and title", () => {
  const markdown = mdxToMarkdown(page(TABS));
  assert.ok(
    markdown.includes('**Python**\n\n*quick_start.py*\n\n```python\nprint("FY2023")\n```'),
    markdown,
  );
  assert.ok(markdown.includes('**JavaScript**\n\n```js\nconsole.log("FY2023");\n```'), markdown);
  assert.doesNotMatch(markdown, /<Tabs|<TabItem|<Code|^import /m);
});

test("an aside becomes a blockquote that starts with its type and title", () => {
  const markdown = mdxToMarkdown(page('<Aside type="caution" title="Extra week">Week 53 goes in period 12.</Aside>\n'));
  assert.ok(markdown.includes("> **Caution: Extra week**\n>\n> Week 53 goes in period 12."), markdown);
});

test("relative links become absolute URLs on the site", () => {
  const markdown = mdxToMarkdown(
    page('See [the spec](../../reference/spec/) and [errors](#errors).\n\n<LinkCard title="API" href="/reference/api/" />\n'),
  );
  assert.ok(markdown.includes("[the spec](https://fiscalclock.karthik.nz/reference/spec/)"), markdown);
  assert.ok(markdown.includes("[errors](https://fiscalclock.karthik.nz/start-here/quick-start/#errors)"), markdown);
  assert.ok(markdown.includes("[API](https://fiscalclock.karthik.nz/reference/api/)"), markdown);
});

test("HTML elements keep their text", () => {
  assert.match(mdxToMarkdown(page('Ends on <time datetime="2024-02-03">3 Feb 2024</time>.\n')), /Ends on 3 Feb 2024\./);
});

test("a component with no conversion fails and names itself and the page", () => {
  assert.throws(
    () => mdxToMarkdown(page('<Steps>\n\n1. Install.\n\n</Steps>\n')),
    /start-here\/quick-start: no Markdown conversion for the component <Steps>/,
  );
});

test("a Code component whose code is not a ?raw import fails", () => {
  assert.throws(() => mdxToMarkdown(page('<Code code={example} lang="python" />\n')), /must name a default import ending in \?raw/);
});
