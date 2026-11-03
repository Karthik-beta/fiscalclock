import type { Blockquote, Code, Paragraph, Parent, PhrasingContent, Root, RootContent, Text } from "mdast";
import type { MdxJsxFlowElement, MdxJsxTextElement } from "mdast-util-mdx-jsx";
import remarkGfm from "remark-gfm";
import remarkMdx from "remark-mdx";
import remarkParse from "remark-parse";
import remarkStringify from "remark-stringify";
import { unified } from "unified";
import { HERO_SUMMARY, INSTALL_COMMANDS } from "../lib/hero.ts";

export interface MdxPage {
  /** Names the page in error messages. */
  id: string;
  title: string;
  description?: string;
  /** The source without its frontmatter, and whether it is MDX or plain Markdown. */
  body: string;
  format: "mdx" | "md";
  /** Absolute URL of the page's HTML version; relative links resolve against it. */
  url: string;
  /** Returns the contents of a file imported with `?raw`, given the specifier without `?raw`. */
  readRaw: (specifier: string) => string;
  /** Returns the exports of a relative module the page imports, such as `../../lib/landing`. */
  readModule: (specifier: string) => Promise<Record<string, unknown>>;
}

type JsxElement = MdxJsxFlowElement | MdxJsxTextElement;

interface Context {
  page: MdxPage;
  /** What each name the page imports stands for in an expression. */
  scope: Map<string, unknown>;
}

type ToMarkdown = (element: JsxElement, children: RootContent[], context: Context) => RootContent[];

/**
 * How each MDX component becomes plain Markdown, keyed by the name a page uses.
 * A component missing from this table fails the build, so a page cannot start
 * using one whose content would silently vanish from its Markdown copy. HTML
 * elements not listed here keep their content and lose the tag.
 */
export const COMPONENTS: Record<string, ToMarkdown> = {
  Code(element, _children, context) {
    const code: Code = {
      type: "code",
      lang: valueProp(element, "lang", context) ?? null,
      value: requiredValueProp(element, "code", context).replace(/\n$/, ""),
    };
    const title = valueProp(element, "title", context);
    return title ? [caption(title), code] : [code];
  },
  Tabs: (_element, children) => children,
  TabItem(element, children, context) {
    const label = requiredValueProp(element, "label", context);
    return [paragraph({ type: "strong", children: [{ type: "text", value: label }] }), ...children];
  },
  Aside(element, children, context) {
    const type = valueProp(element, "type", context) ?? "note";
    const title = valueProp(element, "title", context);
    const heading = capitalise(type) + (title ? `: ${title}` : "");
    return [
      {
        type: "blockquote",
        children: [paragraph({ type: "strong", children: [{ type: "text", value: heading }] }), ...(children as Blockquote["children"])],
      },
    ];
  },
  a(element, children, context) {
    const href = requiredValueProp(element, "href", context);
    return [{ type: "link", url: absolute(href, context.page), children: inline(children) }];
  },
  RawOutput(element, _children, context) {
    const text = requiredValueProp(element, "text", context);
    return [caption(requiredValueProp(element, "label", context)), { type: "code", lang: "text", value: text.replace(/\n$/, "") }];
  },
  YearStripFigure: (_element, children) => children,
  // The hero's lookup needs a browser; its Markdown is the summary and the install commands.
  Hero: (_element, _children, context) => [
    paragraph({ type: "text", value: HERO_SUMMARY }),
    { type: "code", lang: "sh", value: INSTALL_COMMANDS.map(({ command }) => command).join("\n") },
    paragraph({ type: "link", url: absolute("/playground/", context.page), children: [{ type: "text", value: "Try a date in the playground" }] }),
  ],
};

const PARSERS = {
  mdx: unified().use(remarkParse).use(remarkMdx).use(remarkGfm),
  md: unified().use(remarkParse).use(remarkGfm),
};
// No remark-mdx here: an MDX node that survives conversion makes stringify throw.
// Unaligned tables: padding cells to align pipes doubled the size of the spec pages.
const printer = unified().use(remarkGfm, { tablePipeAlign: false }).use(remarkStringify, { bullet: "-", emphasis: "*", fences: true, rule: "-" });

/** Converts a docs page to plain Markdown, headed by its title and description. */
export async function mdxToMarkdown(page: MdxPage): Promise<string> {
  const tree = PARSERS[page.format].parse(page.body) as Root;
  const context: Context = { page, scope: await imports(tree, page) };
  const head: RootContent[] = [{ type: "heading", depth: 1, children: [{ type: "text", value: page.title }] }];
  if (page.description) head.push(paragraph({ type: "text", value: page.description }));
  const root: Root = { type: "root", children: [...head, ...convert(tree.children, context)] };
  return printer.stringify(root);
}

function convert(nodes: RootContent[], context: Context): RootContent[] {
  return nodes.flatMap((node): RootContent[] => {
    if ("children" in node) {
      (node as Parent).children = convert((node as Parent).children, context) as Parent["children"];
    }
    switch (node.type) {
      case "mdxjsEsm":
        return [];
      case "mdxFlowExpression":
      case "mdxTextExpression": {
        const value = evaluate(node.value, context);
        if (value === undefined) return [];
        const text: Text = { type: "text", value };
        return node.type === "mdxFlowExpression" ? [paragraph(text)] : [text];
      }
      case "mdxJsxFlowElement":
      case "mdxJsxTextElement":
        return element(node, context);
      case "link":
      case "image":
      case "definition":
        node.url = absolute(node.url, context.page);
        return [node];
      case "html":
        return /^<!--[\s\S]*-->$/.test(node.value.trim()) ? [] : [node];
      case "paragraph":
        // A component on one line, such as <Aside>text</Aside>, parses as an
        // inline element in a paragraph, so its block output lands inside one.
        return node.children.every((child) => PHRASING.has(child.type)) ? [node] : asBlocks(node.children);
      default:
        return [node];
    }
  });
}

function element(node: JsxElement, context: Context): RootContent[] {
  const flow = node.type === "mdxJsxFlowElement";
  const children = node.children as RootContent[];
  const toMarkdown = node.name === null ? undefined : COMPONENTS[node.name];
  if (toMarkdown) {
    const result = toMarkdown(node, flow ? asBlocks(children) : children, context);
    return flow ? asBlocks(result) : result;
  }
  if (node.name === null || /^[a-z]/.test(node.name)) return flow ? asBlocks(children) : children;
  throw new Error(
    `${context.page.id}: no Markdown conversion for the component <${node.name}>. Add it to COMPONENTS in src/agents/mdx-to-markdown.ts.`,
  );
}

const PHRASING = new Set([
  "text", "emphasis", "strong", "delete", "inlineCode", "link", "linkReference",
  "image", "imageReference", "break", "footnoteReference", "html",
]);

/** Wraps each run of inline nodes in a paragraph, so block content holds only blocks. */
function asBlocks(nodes: RootContent[]): RootContent[] {
  const blocks: RootContent[] = [];
  let run: Paragraph | undefined;
  for (const node of nodes) {
    if (!PHRASING.has(node.type)) {
      blocks.push(node);
      run = undefined;
    } else if (run) {
      run.children.push(node as PhrasingContent);
    } else {
      run = { type: "paragraph", children: [node as PhrasingContent] };
      blocks.push(run);
    }
  }
  return blocks;
}

/** The inline content of nodes, with paragraphs unwrapped. */
function inline(nodes: RootContent[]): PhrasingContent[] {
  return nodes.flatMap((node) => (node.type === "paragraph" ? node.children : [node as PhrasingContent]));
}

/**
 * Binds the imports a page can use in an expression: a `?raw` import to the
 * file's text, and a relative module to its exports. Components, styles and
 * packages are left out; their names are only valid as elements.
 */
async function imports(tree: Root, page: MdxPage): Promise<Map<string, unknown>> {
  const scope = new Map<string, unknown>();
  for (const node of tree.children) {
    if (node.type !== "mdxjsEsm") continue;
    for (const [, star, name, specifier] of node.value.matchAll(/import\s+(\*\s+as\s+)?(\w+)\s+from\s+["']([^"']+)["']/g)) {
      if (specifier!.endsWith("?raw")) {
        scope.set(name!, page.readRaw(specifier!.slice(0, -"?raw".length)));
      } else if (specifier!.startsWith(".") && !/\.(astro|css)$/.test(specifier!)) {
        const module = await page.readModule(specifier!);
        scope.set(name!, star ? module : module.default);
      }
    }
  }
  return scope;
}

/** Evaluates what a page may write in braces: a comment, a string literal or a dotted name from its imports. */
function evaluate(expression: string, context: Context): string | undefined {
  const source = expression.trim();
  if (/^\/\*[\s\S]*\*\/$/.test(source)) return undefined;
  const literal = /^(["'])(.*)\1$/s.exec(source);
  if (literal) return literal[2];
  if (/^[A-Za-z_$][\w$]*(\.[A-Za-z_$][\w$]*)*$/.test(source)) {
    const [name, ...path] = source.split(".");
    let value: unknown = context.scope.get(name!);
    for (const key of path) value = (value as Record<string, unknown> | undefined)?.[key];
    if (typeof value === "string" || typeof value === "number") return String(value);
  }
  throw new Error(
    `${context.page.id}: the expression {${source}} is not a comment, a string, a ?raw import or a value from an imported module`,
  );
}

function prop(element: JsxElement, name: string) {
  return element.attributes.find((a) => a.type === "mdxJsxAttribute" && a.name === name)?.value;
}

/** An attribute's value; an `{expression}` value is evaluated. */
function valueProp(element: JsxElement, name: string, context: Context): string | undefined {
  const value = prop(element, name);
  if (value === undefined || value === null) return undefined;
  return typeof value === "string" ? value : evaluate(value.value, context);
}

function requiredValueProp(element: JsxElement, name: string, context: Context): string {
  const value = valueProp(element, name, context);
  if (value === undefined) throw new Error(`${context.page.id}: <${element.name}> needs a ${name} attribute`);
  return value;
}

function absolute(url: string, page: MdxPage): string {
  return new URL(url, page.url).href;
}

function paragraph(child: PhrasingContent): Paragraph {
  return { type: "paragraph", children: [child] };
}

function caption(text: string): Paragraph {
  return paragraph({ type: "emphasis", children: [{ type: "text", value: text }] });
}

function capitalise(word: string): string {
  return word.charAt(0).toUpperCase() + word.slice(1);
}
