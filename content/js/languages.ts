import { StreamLanguage } from "@codemirror/language";

import { c, csharp, cpp } from "@codemirror/legacy-modes/mode/clike";
import { lua } from "@codemirror/legacy-modes/mode/lua";
import { python } from "@codemirror/legacy-modes/mode/python";
import { knight } from "codemirror-lang-knight";

export const languages = {
  c: StreamLanguage.define(c),
  csharp: StreamLanguage.define(csharp),
  cpp: StreamLanguage.define(cpp),
  knight: knight(),
  lua: StreamLanguage.define(lua),
  python: StreamLanguage.define(python),
};

export enum LanguageName {
  C = "c",
  CSharp = "csharp",
  CPP = "cpp",
  Knight = "knight",
  Lua = "lua",
  Python = "python",
}

export const languageNames: Record<string, LanguageName> = {
  C: LanguageName.C,
  "C#": LanguageName.CSharp,
  "C++": LanguageName.CPP,
  Knight: LanguageName.Knight,
  Lua: LanguageName.Lua,
  Python: LanguageName.Python,
};

export const snippets: Record<LanguageName, string> = {
  [LanguageName.C]: `#include <stdio.h>

int main(int argc, char** argv) {
    puts("Hello, World!");

    for (int i = 0; i < argc; i++) {
      puts(argv[i]);
    }

    return 0;
}`,
  [LanguageName.CSharp]: `System.Console.WriteLine("Hello, World!");

for (int i = 0; i < args.length; i++) {
  System.Console.WriteLine(args[i]);
}`,
  [LanguageName.CPP]: `#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;

    for (int i = 0; i < argc; i++) {
      std::cout << argv[i] << std::endl;
    }

    return 0;
}`,
  [LanguageName.Knight]: `; OUTPUT "Hello, world!"

; = i 0
WHILE > LENGTH _ i
  ; OUTPUT [GET _ i 1
  : = i + i 1`,
  [LanguageName.Lua]: `print("Hello, World!")
  
for i = 0, #arg do
  print(arg[i])
end`,
  [LanguageName.Python]: `print("Hello, World!")
  
for arg in sys.argv:
  print(arg)`,
};