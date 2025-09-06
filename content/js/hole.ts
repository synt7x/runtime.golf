import { Editor } from "./components/editor";
import { Selector } from "./components/selector";
import { LanguageName } from "./languages";
import { syntaxTree } from "@codemirror/language";

const editor = new Editor(document.getElementById("editor") as HTMLElement);
const selector = new Selector(
  document.getElementById("selector") as HTMLSelectElement
);

selector.onChange((language: string) => {
  editor.setLanguage(language as LanguageName);
});
