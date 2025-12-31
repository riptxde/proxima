import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useExecutor } from "./useExecutor";
import { useExecutorClients } from "./useExecutorClients";
import { useLogger } from "@/composables/useLogger";
import { useSettings } from "@/features/settings/composables/useSettings";
import { toast } from "vue-sonner";

interface HttpExecutePayload {
  script: string;
  source: "http_file" | "http_direct";
}

let unlistenFn: UnlistenFn | null = null;

export function useHttpExecutor() {
  const { executeScript } = useExecutor();
  const { getSelectedClientIds } = useExecutorClients();
  const { addLog } = useLogger();
  const { executionSettings } = useSettings();

  const init = async () => {
    if (unlistenFn) return; // Already initialized

    unlistenFn = await listen<HttpExecutePayload>(
      "http-execute-script",
      async (event) => {
        const { script, source } = event.payload;

        // Check if HTTP request execution is enabled
        if (!executionSettings.value.httpRequestExecution) {
          addLog(
            "warning",
            "An HTTP script execution request was made, but HTTP request execution is disabled",
          );
          toast.warning("HTTP execution denied", {
            description: "HTTP request execution is disabled in settings.",
          });
          return;
        }

        const clientIds = getSelectedClientIds();

        addLog("info", `HTTP request received: ${source}`);

        // Execute using normal flow (handles validation, logging, toasts)
        await executeScript(script, clientIds);
      },
    );
  };

  const cleanup = () => {
    if (unlistenFn) {
      unlistenFn();
      unlistenFn = null;
    }
  };

  return {
    init,
    cleanup,
  };
}
