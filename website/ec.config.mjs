import { defineEcConfig, ExpressiveCodeTheme } from "@astrojs/starlight/expressive-code";
// Imported rather than read from disk: the <Code> component loads this file
// from a bundled chunk, where a path built from import.meta.url does not exist.
import light from "./src/styles/generated/tokens.light.json" with { type: "json" };
import dark from "./src/styles/generated/tokens.dark.json" with { type: "json" };

// Syntax colours are the four code tokens, read from the synced token build so
// they cannot drift from design/tokens. Each pair is contrast-checked there.
const SCOPES = {
  keyword: ["keyword", "storage", "storage.type", "storage.modifier", "constant.language", "variable.language"],
  string: ["string", "punctuation.definition.string"],
  comment: ["comment", "punctuation.definition.comment"],
};

function codeTheme(type, tokens) {
  return new ExpressiveCodeTheme({
    name: `fiscalclock-${type}`,
    type,
    colors: {
      "editor.background": tokens["--fc-color-code-bg"],
      "editor.foreground": tokens["--fc-color-code-text"],
    },
    tokenColors: Object.entries(SCOPES).map(([role, scope]) => ({
      scope,
      settings: { foreground: tokens[`--fc-color-code-${role}`] },
    })),
  });
}

export default defineEcConfig({
  themes: [codeTheme("light", light), codeTheme("dark", dark)],
  // Token colours already meet 4.5:1; a higher floor would make Expressive
  // Code shift them away from the token values.
  minSyntaxHighlightingColorContrast: 0,
  styleOverrides: {
    borderRadius: "var(--fc-radius-md)",
    borderColor: "var(--fc-color-border-default)",
    borderWidth: "var(--fc-border-width-thin)",
    codeBackground: "var(--fc-color-code-bg)",
    codeForeground: "var(--fc-color-code-text)",
    codeFontFamily: "var(--fc-type-code-font-family)",
    codeFontSize: "var(--fc-type-code-font-size)",
    codeLineHeight: "var(--fc-type-code-line-height)",
    uiFontFamily: "var(--fc-type-body-font-family)",
    uiFontSize: "var(--fc-type-small-font-size)",
    focusBorder: "var(--fc-color-focus-ring)",
    scrollbarThumbColor: "var(--fc-color-border-strong)",
    scrollbarThumbHoverColor: "var(--fc-color-text-muted)",
    frames: {
      shadowColor: "transparent",
      frameBoxShadowCssValue: "none",
      editorBackground: "var(--fc-color-code-bg)",
      editorTabBarBackground: "var(--fc-color-bg-subtle)",
      editorTabBarBorderColor: "var(--fc-color-border-default)",
      editorTabBarBorderBottomColor: "var(--fc-color-border-default)",
      editorTabBorderRadius: "0",
      editorActiveTabBackground: "var(--fc-color-code-bg)",
      editorActiveTabForeground: "var(--fc-color-text-default)",
      editorActiveTabBorderColor: "transparent",
      editorActiveTabIndicatorTopColor: "var(--fc-color-accent-default)",
      editorActiveTabIndicatorBottomColor: "transparent",
      terminalBackground: "var(--fc-color-code-bg)",
      terminalTitlebarBackground: "var(--fc-color-bg-subtle)",
      terminalTitlebarForeground: "var(--fc-color-text-muted)",
      terminalTitlebarBorderBottomColor: "var(--fc-color-border-default)",
      terminalTitlebarDotsForeground: "var(--fc-color-border-strong)",
      inlineButtonForeground: "var(--fc-color-text-default)",
      inlineButtonBackground: "var(--fc-color-bg-raised)",
      inlineButtonBorder: "var(--fc-color-border-strong)",
      tooltipSuccessBackground: "var(--fc-color-status-success)",
      tooltipSuccessForeground: "var(--fc-color-text-on-accent)",
    },
  },
});
