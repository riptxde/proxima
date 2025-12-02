import { ref, watch, type Ref } from "vue";
import { load, type Store } from "@tauri-apps/plugin-store";
import { invoke } from "@tauri-apps/api/core";
import {
  DEFAULT_SETTINGS,
  type Settings,
  type EditorSettings,
  type ExecutionSettings,
  type ApplicationSettings,
} from "../types/settings";

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

async function initializeStore() {
  if (isInitialized) return;

  try {
    store = await load("settings.json", { autoSave: 100, defaults: {} });

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
    } else {
      // Save defaults if no settings exist
      await saveSettings();
    }

    // Watch for changes and persist
    watch(
      [editorSettings, executionSettings, applicationSettings],
      async () => {
        await saveSettings();
      },
      { deep: true },
    );

    isInitialized = true;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    invoke("add_log", {
      level: 3,
      message: `Settings initialization failed: ${errorMessage}`,
    }).catch(() => {});
  }
}

async function saveSettings() {
  if (!store) return;

  const settings: Settings = {
    editor: editorSettings.value,
    execution: executionSettings.value,
    application: applicationSettings.value,
  };

  await store.set("settings", settings);
}

async function resetSettings() {
  editorSettings.value = { ...DEFAULT_SETTINGS.editor };
  executionSettings.value = { ...DEFAULT_SETTINGS.execution };
  applicationSettings.value = { ...DEFAULT_SETTINGS.application };

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

    // Methods
    resetSettings,
  };
}
