import type * as Monaco from "monaco-editor";

export const robloxDarkTheme: Monaco.editor.IStandaloneThemeData = {
  base: "vs-dark",
  inherit: true,
  rules: [
    // Comments
    { token: "comment", foreground: "666666", fontStyle: "italic" },
    {
      token: "comment.line",
      foreground: "666666",
      fontStyle: "italic",
    },
    {
      token: "comment.block",
      foreground: "666666",
      fontStyle: "italic",
    },

    // Variables
    { token: "variable", foreground: "F86D7C" },
    { token: "variable.name", foreground: "F86D7C" },
    { token: "variable.other", foreground: "F86D7C" },

    // Keywords and control flow
    { token: "keyword", foreground: "F86D7C" },
    { token: "keyword.control", foreground: "F86D7C" },
    { token: "keyword.operator", foreground: "cccccc" },
    { token: "keyword.operator.logical", foreground: "cccccc" },

    // Storage types
    { token: "storage.type", foreground: "cccccc" },
    { token: "storage.modifier", foreground: "cccccc" },

    // Functions
    { token: "entity.name.function", foreground: "84D6F7" },
    { token: "support.function", foreground: "84D6F7" },
    { token: "meta.function-call", foreground: "84D6F7" },

    // Constants and numbers
    { token: "constant", foreground: "FFC600" },
    { token: "constant.numeric", foreground: "FFC600" },
    { token: "constant.language", foreground: "f07178" },
    { token: "constant.character", foreground: "FFC600" },
    { token: "constant.escape", foreground: "FFC600" },

    // Strings
    { token: "string", foreground: "ADF195" },
    { token: "string.quoted", foreground: "ADF195" },

    // Classes and types
    { token: "entity.name.class", foreground: "FFCB6B" },
    { token: "entity.name.type", foreground: "FFCB6B" },
    { token: "support.type", foreground: "F86D7C" },
    { token: "support.class", foreground: "FFCB6B" },

    // Punctuation
    { token: "punctuation", foreground: "cccccc" },
    { token: "punctuation.definition.string", foreground: "ADF195" },

    // Invalid
    { token: "invalid", foreground: "FF0000" },
    { token: "invalid.illegal", foreground: "FF0000" },
  ],
  colors: {
    // Use card background to match file explorer panel
    "editor.background": "#171717",
    "editor.foreground": "#d4e4e4",

    // Selection and highlights
    "editor.lineHighlightBackground": "#2a2a2a",
    "editor.selectionBackground": "#3a3a3a",
    "editor.inactiveSelectionBackground": "#2f2f2f",

    // Gutter
    "editorLineNumber.foreground": "#666666",
    "editorLineNumber.activeForeground": "#bbbbbb",
    "editorGutter.background": "#171717",

    // Cursor
    "editorCursor.foreground": "#eeffff",

    // Scrollbar
    "scrollbarSlider.background": "#2E2E2E",
    "scrollbarSlider.hoverBackground": "#3a3a3a",
    "scrollbarSlider.activeBackground": "#454545",

    // Borders
    "editorWidget.border": "#222222",

    // Suggestions/autocomplete
    "editorSuggestWidget.background": "#272727",
    "editorSuggestWidget.border": "#222222",
    "editorSuggestWidget.selectedBackground": "#232323",

    // Find/replace widget
    "editorWidget.background": "#272727",
  },
};
