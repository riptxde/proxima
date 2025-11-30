<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "vue-sonner";
import { Dock, DockIcon } from "@/components/ui/dock";
import { Play, Eraser, FolderOpen, Save } from "lucide-vue-next";
import DockLiquidGlass from "@/components/DockLiquidGlass.vue";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import { useTabState } from "@/composables/useTabState";
import SaveDialog from "@/components/SaveDialog.vue";

const {
    clearActiveTab,
    openFileAsTab,
    getActiveTabContent,
    getActiveTabFilePath,
    updateActiveTabFilePath,
} = useTabState();

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

        // Parse the filepath to extract filename and folder
        const pathParts = filePath.split(/[\\/]/);
        const filename = pathParts[pathParts.length - 1];
        const folder = pathParts[pathParts.length - 2] as
            | "Scripts"
            | "AutoExec";

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

const handleSave = async (filename: string, folder: "Scripts" | "AutoExec") => {
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
</script>

<template>
    <div class="flex justify-center items-center py-4">
        <!-- Hidden file input -->
        <input
            ref="fileInputRef"
            type="file"
            accept=".lua,.luau,.txt"
            @change="handleFileChange"
            class="hidden"
        />

        <TooltipProvider>
            <DockLiquidGlass>
                <Dock class="m-0!">
                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon>
                                <Play
                                    class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Execute</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon @click="clearActiveTab">
                                <Eraser
                                    class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Clear</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon @click="handleOpenScript">
                                <FolderOpen
                                    class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Open Script</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon @click="handleSaveClick">
                                <Save
                                    class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Save</p>
                        </TooltipContent>
                    </Tooltip>
                </Dock>
            </DockLiquidGlass>
        </TooltipProvider>

        <!-- Save Dialog -->
        <SaveDialog v-model:open="saveDialogOpen" @save="handleSave" />
    </div>
</template>
