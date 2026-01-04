import { ref } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useLogger } from "@/composables/useLogger";

// Shared state
const isLaunching = ref(false);
const queueCount = ref(0);
const launchProgress = ref(0);
const launchStatus = ref("Waiting for Cooldown");
const launchError = ref("");

let unlistenProgressFn: UnlistenFn | null = null;
let unlistenQueueFn: UnlistenFn | null = null;

export function useLauncherProgress() {
  const { addLog } = useLogger();

  const init = async () => {
    if (unlistenProgressFn) return; // Already initialized

    // Listen for launcher progress updates from IPC
    unlistenProgressFn = await listen<{
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
            launchStatus.value = "Waiting for Cooldown";
          }
        }, 3000);
      }
    });

    // Listen for launcher queue updates from backend
    unlistenQueueFn = await listen<{
      count: number;
    }>("launcher-queue-update", (event) => {
      queueCount.value = event.payload.count;
    });
  };

  const cleanup = () => {
    if (unlistenProgressFn) {
      unlistenProgressFn();
      unlistenProgressFn = null;
    }
    if (unlistenQueueFn) {
      unlistenQueueFn();
      unlistenQueueFn = null;
    }
  };

  return {
    // State
    isLaunching,
    queueCount,
    launchProgress,
    launchStatus,
    launchError,

    // Methods
    init,
    cleanup,
  };
}
