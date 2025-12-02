export interface EditorSettings {
  wordWrap: boolean;
  fontSize: number;
  minimap: boolean;
  fontLigatures: boolean;
}

export interface ExecutionSettings {
  autoExecute: boolean;
}

export interface ApplicationSettings {
  alwaysOnTop: boolean;
}

export interface Settings {
  editor: EditorSettings;
  execution: ExecutionSettings;
  application: ApplicationSettings;
}

export const DEFAULT_SETTINGS: Settings = {
  editor: {
    wordWrap: false,
    fontSize: 14,
    minimap: false,
    fontLigatures: true,
  },
  execution: {
    autoExecute: true,
  },
  application: {
    alwaysOnTop: false,
  },
};
