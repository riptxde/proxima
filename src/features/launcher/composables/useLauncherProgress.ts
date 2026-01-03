import { ref } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useLogger } from "@/composables/useLogger";

// Shared state
const isLaunching = ref(false);
const launchProgress = ref(0);
const launchStatus = ref("Ready");
const launchError = ref("");

let unlistenFn: UnlistenFn | null = null;

export function useLauncherProgress() {
  const { addLog } = useLogger();

  const init = async () => {
    if (unlistenFn) return; // Already initialized

    // Listen for launcher progress updates from IPC
    unlistenFn = await listen<{
      progress: number;
      status: string;
      error?: string;
    }>("launcher-progress", (event) => {
      isLaunching.value = true;
      launchProgress.value = event.payload.progress;
      launchStatus.value = event.payload.status;

      // Handle errors
      if (event.payload.error) {
        launchError.value = event.payload.error;
        addLog("error", `Launcher error: ${event.payload.error}`);
      } else {
        launchError.value = "";
      }

      // Reset after completion or error
      if (event.payload.progress >= 100 || event.payload.error) {
        setTimeout(() => {
          isLaunching.value = false;
          launchProgress.value = 0;
          if (!event.payload.error) {
            launchStatus.value = "Ready";
          }
        }, 3000);
      }
    });
  };

  const cleanup = () => {
    if (unlistenFn) {
      unlistenFn();
      unlistenFn = null;
    }
  };

  return {
    // State
    isLaunching,
    launchProgress,
    launchStatus,
    launchError,

    // Methods
    init,
    cleanup,
  };
}
