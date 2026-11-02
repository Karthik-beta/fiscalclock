import type { Blockquote, Code, Paragraph, Parent, Root, RootContent } from "mdast";
import type { MdxJsxFlowElement, MdxJsxTextElement } from "mdast-util-mdx-jsx";
import remarkGfm from "remark-gfm";
import remarkMdx from "remark-mdx";
import remarkParse from "remark-parse";
import remarkStringify from "remark-stringify";
import { unified } from "unified";

export interface MdxPage {
  /** Names the page in error messages. */
  id: string;
  title: string;
  description?: string;
  /** The MDX source without its frontmatter. */
  body: string;
  /** Absolute URL of the page's HTML version; relative links resolve against it. */
  url: string;
  /** Returns the contents of a file imported with `?raw`, given the specifier without `?raw`. */
  readRaw: (specifier: string) => string;
}

type JsxElement = MdxJsxFlowElement | MdxJsxTextElement;

interface Context {
  page: MdxPage;
  /** Identifier of each `import x from "...?raw"` mapped to the imported text. */
  raw: Map<string, string>;
}

type ToMarkdown = (element: JsxElement, children: RootContent[], context: Context) => RootContent[];

/**
 * How each MDX component becomes plain Markdown, keyed by the name a page uses.
 * A component missing from this table fails the build, so a page cannot start
 * using one whose content would silently vanish from its Markdown copy.
 */
export const COMPONENTS: Record<string, ToMarkdown> = {
  Code(element, _children, context) {
    const code: Code = {
      type: "code",
      lang: stringProp(element, "lang", context) ?? null,
      value: codeProp(element, context).replace(/\n$/, ""),
    };
    const title = stringProp(element, "title", context);
    return title ? [paragraph({ type: "emphasis", children: [{ type: "text", value: title }] }), code] : [code];
  },
  Tabs: (_element, children) => children,
  TabItem(element, children, context) {
    const label = requiredStringProp(element, "label", context);
    return [paragraph({ type: "strong", children: [{ type: "text", value: label }] }), ...children];
  },
  Aside(element, children, context) {
    const type = stringProp(element, "type", context) ?? "note";
    const title = stringProp(element, "title", context);
    const heading = capitalise(type) + (title ? `: ${title}` : "");
    return [
      {
        type: "blockquote",
        children: [paragraph({ type: "strong", children: [{ type: "text", value: heading }] }), ...(children as Blockquote["children"])],
      },
    ];
  },
  CardGrid: (_element, children) => children,
  LinkCard(element, _children, context) {
    const title = requiredStringProp(element, "title", context);
    const href = requiredStringProp(element, "href", context);
    const description = stringProp(element, "description", context);
    return [
      {
        type: "paragraph",
        children: [
          { type: "link", url: absolute(href, context.page), children: [{ type: "text", value: title }] },
          ...(description ? [{ type: "text" as const, value: `: ${description}` }] : []),
        ],
      },
    ];
  },
};

const parser = unified().use(remarkParse).use(remarkMdx).use(remarkGfm);
// No remark-mdx here: an MDX node that survives conversion makes stringify throw.
const printer = unified().use(remarkGfm).use(remarkStringify, { bullet: "-", emphasis: "*", fences: true, rule: "-" });

/** Converts an MDX docs page to plain Markdown, headed by its title and description. */
export function mdxToMarkdown(page: MdxPage): string {
  const tree = parser.parse(page.body) as Root;
  const context: Context = { page, raw: rawImports(tree, page) };
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
      case "mdxTextExpression":
        if (/^\s*\/\*[\s\S]*\*\/\s*$/.test(node.value)) return [];
        throw new Error(`${context.page.id}: the expression {${node.value}} has no Markdown form`);
      case "mdxJsxFlowElement":
      case "mdxJsxTextElement":
        return element(node, context);
      case "link":
      case "image":
      case "definition":
        node.url = absolute(node.url, context.page);
        return [node];
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
  // HTML elements (lowercase names) and fragments keep their content.
  if (node.name === null || /^[a-z]/.test(node.name)) return node.children as RootContent[];
  const toMarkdown = COMPONENTS[node.name];
  if (!toMarkdown) {
    throw new Error(
      `${context.page.id}: no Markdown conversion for the component <${node.name}>. Add it to COMPONENTS in src/agents/mdx-to-markdown.ts.`,
    );
  }
  return toMarkdown(node, asBlocks(node.children as RootContent[]), context);
}

const PHRASING = new Set([
  "text", "emphasis", "strong", "delete", "inlineCode", "link", "linkReference",
  "image", "imageReference", "break", "footnoteReference", "html",
]);

/** Wraps each run of inline nodes in a paragraph, so a component's content is always blocks. */
function asBlocks(nodes: RootContent[]): RootContent[] {
  const blocks: RootContent[] = [];
  let run: Paragraph | undefined;
  for (const node of nodes) {
    if (!PHRASING.has(node.type)) {
      blocks.push(node);
      run = undefined;
    } else if (run) {
      run.children.push(node as Paragraph["children"][number]);
    } else {
      run = { type: "paragraph", children: [node as Paragraph["children"][number]] };
      blocks.push(run);
    }
  }
  return blocks;
}

function rawImports(tree: Root, page: MdxPage): Map<string, string> {
  const raw = new Map<string, string>();
  for (const node of tree.children) {
    if (node.type !== "mdxjsEsm") continue;
    for (const [, name, specifier] of node.value.matchAll(/import\s+(\w+)\s+from\s+["']([^"']+)\?raw["']/g)) {
      raw.set(name!, page.readRaw(specifier!));
    }
  }
  return raw;
}

function prop(element: JsxElement, name: string) {
  return element.attributes.find((a) => a.type === "mdxJsxAttribute" && a.name === name)?.value;
}

function stringProp(element: JsxElement, name: string, context: Context): string | undefined {
  const value = prop(element, name);
  if (value === undefined || value === null) return undefined;
  if (typeof value === "string") return value;
  throw new Error(`${context.page.id}: <${element.name} ${name}> must be a string, not {${value.value}}`);
}

function requiredStringProp(element: JsxElement, name: string, context: Context): string {
  const value = stringProp(element, name, context);
  if (value === undefined) throw new Error(`${context.page.id}: <${element.name}> needs a ${name} attribute`);
  return value;
}

function codeProp(element: JsxElement, context: Context): string {
  const value = prop(element, "code");
  if (typeof value === "string") return value;
  const name = value?.value.trim();
  const text = name === undefined ? undefined : context.raw.get(name);
  if (text === undefined) {
    throw new Error(`${context.page.id}: <Code code={${name ?? ""}}> must name a default import ending in ?raw`);
  }
  return text;
}

function absolute(url: string, page: MdxPage): string {
  return new URL(url, page.url).href;
}

function paragraph(child: Paragraph["children"][number]): Paragraph {
  return { type: "paragraph", children: [child] };
}

function capitalise(word: string): string {
  return word.charAt(0).toUpperCase() + word.slice(1);
}
