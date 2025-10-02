import { HighlightStyle } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";
/*
/*
vim.opt.guifont = "MonaspiceNe NF:h14"


-- ~/.config/nvim/lua/colors/mytheme.lua

vim.cmd("highlight clear")
vim.o.background = "dark"
vim.cmd("syntax reset")
vim.g.colors_name = "synt7x"

-- Base colors
--editor-bg = "#161616"
--editor-fg = "#cccccc"
--editor-grey = "#5f5f5f"
--editor-comment = grey 
--editor-red = "#CC6674"
--editor-green = "#94DD8E"
--editor-yellow = "#D0B06C"
--editor-blue = "#8BADCD"
--editor-magenta = "#B799C8"
--editor-cyan = "#8BADCD"
--editor-orange = "#ffb86c"


-- Optional: differentiate operator types if available

-- UI
vim.api.nvim_set_hl(0, "Normal", { fg = fg, bg = bg })
vim.api.nvim_set_hl(0, "NormalFloat", { fg = fg, bg = bg })
vim.api.nvim_set_hl(0, "FloatBorder", { fg = gray, bg = bg })
vim.api.nvim_set_hl(0, "LineNr", { fg = comment, bg = bg })
vim.api.nvim_set_hl(0, "CursorLineNr", { fg = fg, bg = bg })
vim.api.nvim_set_hl(0, "SignColumn", { bg = bg })
vim.api.nvim_set_hl(0, "StatusLine", { fg = fg, bg = bg })
vim.api.nvim_set_hl(0, "VertSplit", { fg = "#2e2e2e", bg = bg })
vim.api.nvim_set_hl(0, "Special", { fg = blue })

-- Syntax
vim.api.nvim_set_hl(0, "Comment", { fg = comment })
vim.api.nvim_set_hl(0, "String", { fg = yellow })
vim.api.nvim_set_hl(0, "Function", { fg = blue })
vim.api.nvim_set_hl(0, "Keyword", { fg = red })
vim.api.nvim_set_hl(0, "Number", { fg = orange })
vim.api.nvim_set_hl(0, "Boolean", { fg = orange })
vim.api.nvim_set_hl(0, "Type", { fg = cyan })
vim.api.nvim_set_hl(0, "Constant", { fg = magenta })

-- Treesitter
vim.api.nvim_set_hl(0, "@function", { fg = blue })
vim.api.nvim_set_hl(0, "@function.builtin", { fg = blue })
vim.api.nvim_set_hl(0, "@keyword", { fg = red })
vim.api.nvim_set_hl(0, "@comment", { fg = comment })
vim.api.nvim_set_hl(0, "@string", { fg = yellow })
vim.api.nvim_set_hl(0, "@number", { fg = orange })
vim.api.nvim_set_hl(0, "@type", { fg = cyan })
vim.api.nvim_set_hl(0, "@constant", { fg = magenta })


vim.api.nvim_set_hl(0, "@variable", { fg = fg  })            -- white
vim.api.nvim_set_hl(0, "@field", { fg = blue })               -- sky blue
vim.api.nvim_set_hl(0, "@property", { fg = magenta })            -- pinkish
vim.api.nvim_set_hl(0, "@function.call", { fg = green  })       -- light green
vim.api.nvim_set_hl(0, "@method.call", { fg = cyan })         -- teal


vim.api.nvim_set_hl(0, "@punctuation.bracket", { fg = grey })   -- grey braces
vim.api.nvim_set_hl(0, "@punctuation.delimiter", { fg = grey }) -- grey commas, colons
vim.api.nvim_set_hl(0, "@punctuation.special", { fg = grey })   -- for bonus things

vim.api.nvim_set_hl(0, "@operator", { fg = red }) 

vim.api.nvim_set_hl(0, "@operator.dereference", { fg = orange })
vim.api.nvim_set_hl(0, "@punctuation.special", { fg = grey }) -- fallback for odd symbols
-- Just in case Lua is using this group for curly braces
vim.api.nvim_set_hl(0, "@constructor", { fg = grey })

-- Telescope
vim.api.nvim_set_hl(0, "TelescopeNormal", { fg = fg, bg = bg })
vim.api.nvim_set_hl(0, "TelescopeSelection", { fg = fg, bg = "#2a2a2a" })
vim.api.nvim_set_hl(0, "TelescopeBorder", { fg = gray, bg = bg })

-- Diagnostics
vim.api.nvim_set_hl(0, "DiagnosticError", { fg = red })
vim.api.nvim_set_hl(0, "DiagnosticWarn", { fg = orange })
vim.api.nvim_set_hl(0, "DiagnosticInfo", { fg = blue })
vim.api.nvim_set_hl(0, "DiagnosticHint", { fg = cyan })
*/


export const highlighter = HighlightStyle.define([
    {
        tag: [t.keyword, t.operator, t.operatorKeyword, t.modifier, t.controlKeyword],
        color: "var(--editor-red)",
    },
    {
        tag: [t.attributeValue, t.standard(t.name), t.special(t.string)],
        color: "var(--editor-green)",
    },
    {
        tag: [t.deleted, t.character, t.propertyName, t.macroName],
        color: "var(--editor-blue)",
    },
    {
        tag: [t.function(t.variableName), t.definition(t.function(t.name)), t.labelName],
        color: "var(--editor-green)",
    },
    {
        tag: [t.color, t.constant(t.name), t.standard(t.name)],
        color: "var(--editor-magenta)",
    },
    {
        tag: [t.definition(t.name), t.separator],
        color: "var(--editor-cyan)",
    },
    {
        tag: [t.typeName, t.className, t.changed, t.annotation, t.number],
        color: "var(--editor-magenta)",
    },
    {
        tag: [t.operator, t.operatorKeyword],
        color: "var(--editor-orange)",
    },
    {
        tag: [t.url, t.escape, t.regexp, t.link],
        color: "var(--editor-purple)",
    },
    {
        tag: [t.meta, t.comment],
        color: "var(--editor-grey)",
    },
    {
        tag: t.strong,
        fontWeight: "bold",
    },
    {
        tag: t.emphasis,
        fontStyle: "italic",
    },
    {
        tag: t.strikethrough,
        textDecoration: "line-through",
    },
    {
        tag: t.link,
        textDecoration: "underline",
    },
    {
        tag: [t.atom, t.bool, t.special(t.variableName)],
        color: "var(--editor-orange)",
    },
    {
        tag: [t.processingInstruction, t.string, t.inserted],
        color: "var(--editor-yellow)",
    },
    {
        tag: t.invalid,
        color: "var(--editor-red)",
        textDecoration: "underline wavy var(--editor-red)",
    }
])