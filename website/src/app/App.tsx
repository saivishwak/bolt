import React, { useRef } from 'react';
import './App.css';

import Editor, { Monaco } from '@monaco-editor/react';
import langaugeDef from './languageDef';
import theme from './theme';

function App() {
  const editorRef = useRef(null);
  function handleEditorWillMount(monaco: Monaco) {
    // Register a new language
    monaco.languages.register({ id: "bolt" });
    // Register a tokens provider for the language
    monaco.languages.setMonarchTokensProvider("bolt", langaugeDef);
    // Define a new theme that contains only rules that match this language
    monaco.editor.defineTheme("boltTheme", theme);
  }

  function handleEditorDidMount(editor: any, monaco: any) {
    editorRef.current = editor;
  }

  return (
    <div style={{ height: '100vh' }}>
      <h1 className="text-3xl font-bold">
        Bolt Playground
      </h1>
      <Editor
        defaultLanguage="bolt"
        defaultValue="let a = 10;"
        onMount={handleEditorDidMount}
        beforeMount={handleEditorWillMount}
        theme="boltTheme"
      />
    </div>
  );
}

export default App;
