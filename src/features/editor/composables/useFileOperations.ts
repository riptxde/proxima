import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "vue-sonner";
import { useEditorTabs } from "./useEditorTabs";

export function useFileOperations() {
  const {
    openFileAsTab,
    getActiveTabContent,
    getActiveTabFilePath,
    updateActiveTabFilePath,
  } = useEditorTabs();

  const fileInputRef = ref<HTMLInputElement | null>(null);
  const saveDialogOpen = ref(false);

  const handleOpenScript = () => {
    fileInputRef.value?.click();
  };

  const handleFileChange = async (event: Event) => {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];

    if (!file) return;

    const content = await file.text();
    openFileAsTab(file.name, content);

    // Reset input so the same file can be opened again
    input.value = "";
  };

  const handleSaveClick = () => {
    const existingFilePath = getActiveTabFilePath();

    if (existingFilePath) {
      // Tab already has a filepath, save directly
      handleDirectSave(existingFilePath);
    } else {
      // No filepath, show dialog
      saveDialogOpen.value = true;
    }
  };

  const handleDirectSave = async (filePath: string) => {
    try {
      const content = getActiveTabContent();

      // Parse the filepath to extract filename and full folder path
      const pathParts = filePath.split(/[\\/]/);
      const filename = pathParts[pathParts.length - 1];

      // Get everything except the filename as the folder path
      const folder = pathParts.slice(0, -1).join("/");

      const relativePath = await invoke<string>("save_file", {
        filename,
        folder,
        content,
      });

      toast.success("File saved successfully", {
        description: relativePath,
      });
    } catch (error) {
      toast.error("Failed to save file", {
        description: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const handleSave = async (
    filename: string,
    folder: "Scripts" | "AutoExec",
  ) => {
    try {
      const content = getActiveTabContent();

      const relativePath = await invoke<string>("save_file", {
        filename,
        folder,
        content,
      });

      // Update the active tab's filepath
      updateActiveTabFilePath(relativePath);

      toast.success("File saved successfully", {
        description: relativePath,
      });
    } catch (error) {
      toast.error("Failed to save file", {
        description: error instanceof Error ? error.message : String(error),
      });
    }
  };

  return {
    fileInputRef,
    saveDialogOpen,
    handleOpenScript,
    handleFileChange,
    handleSaveClick,
    handleSave,
  };
}
