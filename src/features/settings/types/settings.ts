export interface EditorSettings {
  wordWrap: boolean;
  minimap: boolean;
  fontSize: number;
  font: string;
  fontLigatures: boolean;
  smoothCursor: boolean;
  smoothCursorBlink: boolean;
}

export interface ExecutionSettings {
  autoExecute: boolean;
  httpRequestExecution: boolean;
}

export interface ApplicationSettings {
  alwaysOnTop: boolean;
}

export interface LauncherSettings {
  channel: string;
  versionOverride: string;
  cooldown: number;
  multiInstance: boolean;
}

export interface Settings {
  editor: EditorSettings;
  execution: ExecutionSettings;
  application: ApplicationSettings;
  launcher: LauncherSettings;
}

export const DEFAULT_SETTINGS: Settings = {
  editor: {
    wordWrap: false,
    minimap: false,
    fontSize: 14,
    font: "Lilex",
    fontLigatures: false,
    smoothCursor: true,
    smoothCursorBlink: true,
  },
  execution: {
    autoExecute: true,
    httpRequestExecution: false,
  },
  application: {
    alwaysOnTop: false,
  },
  launcher: {
    channel: "",
    versionOverride: "",
    cooldown: 60,
    multiInstance: false,
  },
};
