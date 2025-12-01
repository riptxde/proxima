import { invoke } from "@tauri-apps/api/core";
import { toast } from "vue-sonner";
import type { ExecuteRequest } from "../types/executor";

export function useExecutor() {
  const executeScript = async (
    script: string,
    clientIds: string[],
  ): Promise<boolean> => {
    // Validate script is not empty
    if (!script || script.trim().length === 0) {
      toast.error("Cannot execute script", {
        description: "Script content is empty",
      });
      return false;
    }

    // Validate at least one client is selected
    if (clientIds.length === 0) {
      toast.error("Cannot execute script", {
        description:
          "No attached clients found, or none are selected for execution",
      });
      return false;
    }

    try {
      const request: ExecuteRequest = {
        client_ids: clientIds,
        script: script,
      };

      await invoke("execute_script", { request });

      toast.success("Script executed", {
        description: `Sent to ${clientIds.length} client${
          clientIds.length !== 1 ? "s" : ""
        }`,
      });
      return true;
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : String(error);
      toast.error("Execution failed", {
        description: errorMessage,
      });
      return false;
    }
  };

  return {
    executeScript,
  };
}
