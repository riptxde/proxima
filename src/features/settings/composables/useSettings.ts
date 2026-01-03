import { ref, watch, type Ref } from "vue";
import { load, type Store } from "@tauri-apps/plugin-store";
import {
  DEFAULT_SETTINGS,
  type Settings,
  type EditorSettings,
  type ExecutionSettings,
  type ApplicationSettings,
  type LauncherSettings,
} from "../types/settings";
import { useLogger } from "@/composables/useLogger";
import { getScriptsPath } from "@/utils/paths";
import { join } from "@tauri-apps/api/path";

let store: Store | null = null;
let isInitialized = false;

// Reactive settings state
const editorSettings: Ref<EditorSettings> = ref({ ...DEFAULT_SETTINGS.editor });
const executionSettings: Ref<ExecutionSettings> = ref({
  ...DEFAULT_SETTINGS.execution,
});
const applicationSettings: Ref<ApplicationSettings> = ref({
  ...DEFAULT_SETTINGS.application,
});
const launcherSettings: Ref<LauncherSettings> = ref({
  ...DEFAULT_SETTINGS.launcher,
});

async function initializeStore() {
  if (isInitialized) return;

  const { addLog } = useLogger();

  try {
    // Get the base directory (same as scripts/autoexec location)
    const basePath = await getScriptsPath();
    const storePath = await join(basePath, "settings.json");

    store = await load(storePath, { autoSave: 100, defaults: {} });

    // Load existing settings or use defaults
    const savedSettings = await store.get<Settings>("settings");

    if (savedSettings) {
      editorSettings.value = {
        ...DEFAULT_SETTINGS.editor,
        ...savedSettings.editor,
      };
      executionSettings.value = {
        ...DEFAULT_SETTINGS.execution,
        ...savedSettings.execution,
      };
      applicationSettings.value = {
        ...DEFAULT_SETTINGS.application,
        ...savedSettings.application,
      };
      launcherSettings.value = {
        ...DEFAULT_SETTINGS.launcher,
        ...savedSettings.launcher,
      };
    } else {
      // Save defaults if no settings exist
      await saveSettings();
    }

    // Watch for changes and persist
    watch(
      [
        editorSettings,
        executionSettings,
        applicationSettings,
        launcherSettings,
      ],
      async () => {
        await saveSettings();
      },
      { deep: true },
    );

    isInitialized = true;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    addLog("error", `Settings initialization failed: ${errorMessage}`);
  }
}

async function saveSettings() {
  if (!store) return;

  const settings: Settings = {
    editor: editorSettings.value,
    execution: executionSettings.value,
    application: applicationSettings.value,
    launcher: launcherSettings.value,
  };

  await store.set("settings", settings);
}

async function resetSettings() {
  editorSettings.value = { ...DEFAULT_SETTINGS.editor };
  executionSettings.value = { ...DEFAULT_SETTINGS.execution };
  applicationSettings.value = { ...DEFAULT_SETTINGS.application };
  launcherSettings.value = { ...DEFAULT_SETTINGS.launcher };

  await saveSettings();
}

export function useSettings() {
  // Initialize on first use
  if (!isInitialized) {
    initializeStore();
  }

  return {
    // Editor settings
    editorSettings,

    // Execution settings
    executionSettings,

    // Application settings
    applicationSettings,

    // Launcher settings
    launcherSettings,

    // Methods
    resetSettings,
  };
}
