import { StreamLanguage } from "@codemirror/language";

import { c, csharp, cpp } from "@codemirror/legacy-modes/mode/clike";
import { lua } from "@codemirror/legacy-modes/mode/lua";
import { python } from "@codemirror/legacy-modes/mode/python";

export const languages = {
    c: StreamLanguage.define(c),
    csharp: StreamLanguage.define(csharp),
    cpp: StreamLanguage.define(cpp),
    fennel: StreamLanguage.define(lua),
    knight: null,
    lua: StreamLanguage.define(lua),
    python: StreamLanguage.define(python),
}

export enum LanguageName {
    C = 'c',
    CSharp = 'csharp',
    CPP = 'cpp',
    Fennel = 'fennel',
    Knight = 'knight',
    Lua = 'lua',
    Python = 'python',
}

export const languageNames: Record<string, LanguageName> = {
    'C': LanguageName.C,
    'C#': LanguageName.CSharp,
    'C++': LanguageName.CPP,
    'Fennel': LanguageName.Fennel,
    'Knight': LanguageName.Knight,
    'Lua': LanguageName.Lua,
    'Python': LanguageName.Python,
};