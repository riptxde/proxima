import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useExecutor } from "./useExecutor";
import { useClients } from "./useClients";
import { useLogger } from "@/composables/useLogger";

interface HttpExecutePayload {
  script: string;
  source: "http_file" | "http_direct";
}

let unlistenFn: UnlistenFn | null = null;

export function useHttpExecutor() {
  const { executeScript } = useExecutor();
  const { getSelectedClientIds } = useClients();
  const { addLog } = useLogger();

  const initialize = async () => {
    if (unlistenFn) return; // Already initialized

    unlistenFn = await listen<HttpExecutePayload>(
      "http-execute-script",
      async (event) => {
        const { script, source } = event.payload;
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
    initialize,
    cleanup,
  };
}
