import type monaco from 'monaco-editor';
const theme: monaco.editor.IStandaloneThemeData = {
    "base": "vs",
    "inherit": true,
    "rules": [
        {
            "background": "ffffff",
            "token": ""
        },
        {
            "foreground": "6a737d",
            "token": "string.comment"
        },
        {
            "foreground": "0000ff",
            "token": "keyword"
        },
        {
            "foreground": "032f62",
            "token": "string"
        },
        {
            "foreground": "24292e",
            "token": "variable.other"
        },
        {
            "foreground": "b31d28",
            "fontStyle": "bold italic underline",
            "token": "invalid.broken"
        },
        {
            "foreground": "b31d28",
            "fontStyle": "bold italic underline",
            "token": "invalid.deprecated"
        },
        {
            "foreground": "fafbfc",
            "background": "b31d28",
            "fontStyle": "italic underline",
            "token": "invalid.illegal"
        },
        {
            "foreground": "b31d28",
            "token": "message.error"
        },
        {
            "foreground": "586069",
            "token": "brackethighlighter.curly"
        },
        {
            "foreground": "586069",
            "token": "brackethighlighter.round"
        },
        {
            "foreground": "586069",
            "token": "brackethighlighter.square"
        },
        {
            "foreground": "586069",
            "token": "brackethighlighter.angle"
        },
        {
            "foreground": "586069",
            "token": "brackethighlighter.quote"
        },
        {
            "foreground": "b31d28",
            "token": "brackethighlighter.unmatched"
        },
    ],
    "colors": {
        "editor.foreground": "#24292e",
        "editor.background": "#ffffff",
        "editor.selectionBackground": "#c8c8fa",
        "editor.inactiveSelectionBackground": "#fafbfc",
        "editor.lineHighlightBackground": "#fafbfc",
        "editorCursor.foreground": "#24292e",
        "editorWhitespace.foreground": "#959da5",
        "editorIndentGuide.background": "#959da5",
        "editorIndentGuide.activeBackground": "#24292e",
        "editor.selectionHighlightBorder": "#fafbfc"
    }

}


export default theme;