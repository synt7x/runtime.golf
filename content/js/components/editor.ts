import { EditorView } from "codemirror";
import { lineNumbers } from "@codemirror/view";
import { Compartment } from "@codemirror/state";
import { languages, languageNames, LanguageName } from "../languages";
import { highlighter } from "../theme";
import { syntaxHighlighting } from "@codemirror/language";

export class Editor {
    editor: HTMLElement;
    language: LanguageName;
    view: EditorView;
    highlighter: Compartment;
    theme: any;

    constructor(editor: HTMLElement) {
        this.editor = editor;
        this.language = languageNames.C;
        this.highlighter = new Compartment();

        this.theme = EditorView.theme({
            "&": {
                color: "var(--text-primary)",
                backgroundColor: "var(--bg-primary)",
                height: "100%"
            },
            ".cm-content": {
                caretColor: "var(--text-primary)",
            },
            ".cm-lineNumbers": {
                color: "var(--text-secondary)",
                backgroundColor: "var(--bg-secondary)",
            },
            ".cm-gutters": {
                borderRight: "1px solid var(--border-color)",
            }
        })

        this.view = new EditorView({
            parent: this.editor,
            extensions: [
                this.theme,
                this.highlighter.of(languages[this.language] || languages.c),
                EditorView.lineWrapping,
                lineNumbers(),
                syntaxHighlighting(highlighter, { fallback: true }),
            ],
        });
    }

    setLanguage(language: LanguageName) {
        language = languageNames[language] || language;
        if (language in languages) {
            this.language = language;
            this.view.dispatch({
                effects: this.highlighter.reconfigure(languages[language] || languages.c),
            });
        } else {
            console.error(`Language ${language} is not supported.`);
        }
    }
}