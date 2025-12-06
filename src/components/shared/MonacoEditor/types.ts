import type * as Monaco from "monaco-editor";

export interface MonacoEditorOptions {
  automaticLayout?: boolean;
  formatOnType?: boolean;
  formatOnPaste?: boolean;
  minimap?: {
    enabled: boolean;
  };
  scrollbar?: {
    verticalScrollbarSize: number;
    horizontalScrollbarSize: number;
  };
  fontSize?: number;
  fontFamily?: string;
  lineNumbers?: "on" | "off" | "relative" | "interval";
  roundedSelection?: boolean;
  padding?: {
    top: number;
    bottom: number;
  };
  overviewRulerLanes?: number;
  hideCursorInOverviewRuler?: boolean;
  scrollBeyondLastLine?: boolean;
  cursorSmoothCaretAnimation?: "off" | "explicit" | "on";
  cursorBlinking?: "blink" | "smooth" | "phase" | "expand" | "solid";
  cursorStyle?:
    | "line"
    | "block"
    | "underline"
    | "line-thin"
    | "block-outline"
    | "underline-thin";
}

export type MonacoInstance = Monaco.editor.IStandaloneCodeEditor;
export type MonacoModule = typeof Monaco;
