import { EditorView, basicSetup } from "codemirror";
import { lua } from "@codemirror/legacy-modes/mode/lua";
import { StreamLanguage } from "@codemirror/language";
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";

const fontFamily = "var(--font-family-mono)";

// Create syntax highlighting theme using your CSS variables
const highlightStyle = HighlightStyle.define([
  { tag: t.keyword, color: "var(--color-blue)" },
  { tag: t.string, color: "var(--color-green)" },
  { tag: t.comment, color: "var(--color-grey)", fontStyle: "italic" },
  { tag: t.number, color: "var(--color-magenta)" },
  { tag: t.function(t.variableName), color: "var(--color-cyan)" },
  { tag: t.variableName, color: "var(--color-white)" },
  { tag: t.operator, color: "var(--color-orange)" },
  { tag: t.punctuation, color: "var(--color-grey)" },
  { tag: t.bracket, color: "var(--color-yellow)" },
  { tag: t.propertyName, color: "var(--color-cyan)" },
  { tag: t.className, color: "var(--color-yellow)" },
  { tag: t.typeName, color: "var(--color-blue)" },
  { tag: t.literal, color: "var(--color-orange)" },
]);

const theme = EditorView.theme({
  ".cm-content": {
    fontFamily,
  },
});

const editor = new EditorView({
  doc: `-- Lua code
function helloWorld()
        print("Hello, World!")
end`,
  extensions: [
    basicSetup,
    theme,
    syntaxHighlighting(highlightStyle),
    StreamLanguage.define(lua),
    EditorView.lineWrapping,
  ],
  parent: document.getElementById("editor") as HTMLElement,
});
