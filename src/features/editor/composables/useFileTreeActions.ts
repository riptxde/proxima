import { invoke } from "@tauri-apps/api/core";
import { toast } from "vue-sonner";
import { useLogger } from "@/composables/useLogger";

export function useFileTreeActions() {
  const { addLog } = useLogger();

  /**
   * Open the file location in the system file explorer
   */
  async function openFileLocation(relativePath: string): Promise<void> {
    try {
      await invoke("open_file_location", {
        relativePath,
      });
      toast.success("Opened file/folder location");
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : String(error);
      addLog("error", `Failed to open file/folder location: ${errorMessage}`);
      toast.error("Failed to open file/folder location");
    }
  }

  /**
   * Rename a file or folder
   * @returns The new relative path if successful, null otherwise
   */
  async function renameItem(
    relativePath: string,
    newName: string,
  ): Promise<string | null> {
    try {
      const newPath = await invoke<string>("rename_file", {
        relativePath,
        newName,
      });
      toast.success("Renamed successfully");
      return newPath;
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : String(error);
      addLog("error", `Failed to rename: ${errorMessage}`);
      toast.error(errorMessage);
      return null;
    }
  }

  /**
   * Delete a file or folder
   */
  async function deleteItem(
    relativePath: string,
    isFolder: boolean,
  ): Promise<boolean> {
    try {
      await invoke("delete_file", {
        relativePath,
        isFolder,
      });
      toast.success("Deleted successfully");
      return true;
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : String(error);
      addLog("error", `Failed to delete: ${errorMessage}`);
      toast.error("Failed to delete");
      return false;
    }
  }

  return {
    openFileLocation,
    renameItem,
    deleteItem,
  };
}
