import type { languages } from 'monaco-editor';


const langaugeDef: languages.IMonarchLanguage = {
    defaultToken: 'invalid',
    tokenPostfix: '.bolt',
    keywords: [
        'for', 'if', 'else', 'return', 'true', 'false', "fn", "null", "let"
    ],

    typeKeywords: [],

    operators: [
        '=', '>', '<', '!', '==', '<=', '>=', '!=', '--', '+', '-', '*', '/'
    ],

    // we include these common regular expressions
    symbols: /[=><!~?:&|+\-*\/\^%]+/,

    // C# style strings
    escapes: /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,

    // The main tokenizer for our languages
    tokenizer: {
        root: [
            // identifiers and keywords
            [/[a-z_$][\w$]*/, {
                cases: {
                    '@typeKeywords': 'keyword',
                    '@keywords': 'keyword',
                    '@default': 'variable'
                }
            }],
            [/[a-z][\w\$]*/, 'type.identifier'],

            // whitespace
            { include: '@whitespace' },

            // delimiters and operators
            [/[{}()\[\]]/, 'brackethighlighter.curly'],
            [/@symbols/, {
                cases: {
                    '@operators': 'operator',
                    '@default': ''
                }
            }],

            // numbers
            [/\d*\.\d+([eE][\-+]?\d+)?/, 'number.float'],
            [/\d+/, 'number'],

            // delimiter: after number because of .\d floats
            [/[;,.]/, 'delimiter'],

            // strings
            [/"([^"\\]|\\.)*$/, 'string.invalid'],  // non-teminated string
            [/"/, { token: 'string.quote', bracket: '@open', next: '@string' }],

            // characters
            [/'[^\\']'/, 'string'],
            [/(')(@escapes)(')/, ['string', 'string.escape', 'string']],
            [/'/, 'string.invalid']
        ],

        comment: [
            [/[^\/*]+/, 'comment'],
            [/[\/*]/, 'comment']
        ],

        string: [
            [/[^\\"]+/, 'string'],
            [/@escapes/, 'string.escape'],
            [/\\./, 'string.escape.invalid'],
            [/"/, { token: 'string.quote', bracket: '@close', next: '@pop' }]
        ],

        whitespace: [
            [/[ \t\r\n]+/, 'white'],
            [/\/\*/, 'comment', '@comment'],
            [/\/\/.*$/, 'comment'],
        ],
    },

}

export default langaugeDef;