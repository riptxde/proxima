<script setup lang="ts">
import { h, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Card } from "@/components/ui/card";
import { Tree } from "@/components/ui/file-tree";
import { Spinner } from "@/components/ui/spinner";
import FileExplorerSearch from "./FileExplorerSearch.vue";
import FileTreeItem from "./FileTreeItem.vue";
import FolderTreeItem from "./FolderTreeItem.vue";
import RenameDialog from "./RenameDialog.vue";
import DeleteConfirmDialog from "./DeleteConfirmDialog.vue";
import { useFileExplorer } from "./useFileExplorer";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";
import { useFileTreeActions } from "@/features/editor/composables/useFileTreeActions";
import { useLogger } from "@/composables/useLogger";
import type { FileNode } from "./types";

const { openFile, updateTabFilePath, closeTabByFilePath } = useEditorTabs();
const { addLog } = useLogger();
const { openFileLocation, renameItem, deleteItem } = useFileTreeActions();

const {
    searchQuery,
    debouncedSearchQuery,
    filteredTree,
    expandedItems,
    isLoading,
    isSearching,
    resultsLimited,
} = useFileExplorer();

// Dialog state
const renameDialogOpen = ref(false);
const deleteDialogOpen = ref(false);
const selectedItem = ref<{
    path: string;
    name: string;
    type: "file" | "folder";
} | null>(null);

async function handleFileClick(filePath: string, fileName: string) {
    try {
        const content = await invoke<string>("read_file_content", {
            relativePath: filePath,
        });
        openFile(fileName, content, filePath);
    } catch (error) {
        const errorMessage =
            error instanceof Error ? error.message : String(error);
        addLog("error", `Failed to open file ${fileName}: ${errorMessage}`);
    }
}

function handleOpenLocation(path: string) {
    openFileLocation(path);
}

function handleRenameClick(
    path: string,
    name: string,
    type: "file" | "folder",
) {
    selectedItem.value = { path, name, type };
    renameDialogOpen.value = true;
}

function handleDeleteClick(
    path: string,
    name: string,
    type: "file" | "folder",
) {
    selectedItem.value = { path, name, type };
    deleteDialogOpen.value = true;
}

async function handleRenameConfirm(newName: string) {
    if (!selectedItem.value) return;

    const oldPath = selectedItem.value.path;
    const newPath = await renameItem(oldPath, newName);

    if (newPath) {
        // Update tab file path if the file is currently open
        updateTabFilePath(oldPath, newPath);
    }

    selectedItem.value = null;
}

async function handleDeleteConfirm() {
    if (!selectedItem.value) return;

    const isFolder = selectedItem.value.type === "folder";
    const success = await deleteItem(selectedItem.value.path, isFolder);

    if (success) {
        // Close tab if the file is currently open
        closeTabByFilePath(selectedItem.value.path);
    }

    selectedItem.value = null;
}

function renderNode(node: FileNode): any {
    if (node.type === "folder") {
        return h(
            FolderTreeItem,
            {
                id: node.id,
                name: node.name,
                path: node.path,
                onOpenLocation: () => handleOpenLocation(node.path),
                onRename: () =>
                    handleRenameClick(node.path, node.name, "folder"),
                onDelete: () =>
                    handleDeleteClick(node.path, node.name, "folder"),
            },
            () => node.children.map((child) => renderNode(child)),
        );
    }
    return h(FileTreeItem, {
        id: node.id,
        name: node.name,
        path: node.path,
        onClick: () => handleFileClick(node.path, node.name),
        onOpenLocation: () => handleOpenLocation(node.path),
        onRename: () => handleRenameClick(node.path, node.name, "file"),
        onDelete: () => handleDeleteClick(node.path, node.name, "file"),
    });
}
</script>

<template>
    <Card class="h-full p-2 flex flex-col gap-3">
        <FileExplorerSearch v-model="searchQuery" />

        <div class="flex-1 overflow-auto relative">
            <!-- Loading Overlay -->
            <div
                v-if="isLoading || isSearching"
                class="absolute inset-0 flex items-center justify-center bg-card/80 backdrop-blur-sm z-10"
            >
                <Spinner class="w-8 h-8" />
            </div>

            <!-- File Tree -->
            <div v-if="filteredTree.length > 0" class="space-y-2 min-w-max">
                <Tree
                    :key="debouncedSearchQuery"
                    initial-selected-id=""
                    :initial-expanded-items="expandedItems"
                >
                    <component
                        v-for="node in filteredTree"
                        :key="node.id"
                        :is="renderNode(node)"
                    />
                </Tree>
                <div
                    v-if="resultsLimited"
                    class="text-muted-foreground text-xs px-4 pb-2 text-center"
                >
                    Showing first 100 results. Refine your search for more
                    specific matches.
                </div>
            </div>

            <!-- Empty State -->
            <div
                v-else-if="!isLoading"
                class="text-muted-foreground text-sm p-4 text-center"
            >
                {{ searchQuery ? "No files found" : "No files" }}
            </div>
        </div>

        <!-- Dialogs -->
        <RenameDialog
            v-model:open="renameDialogOpen"
            :current-name="selectedItem?.name ?? ''"
            :type="selectedItem?.type ?? 'file'"
            @rename="handleRenameConfirm"
        />

        <DeleteConfirmDialog
            v-model:open="deleteDialogOpen"
            :item-name="selectedItem?.name ?? ''"
            :type="selectedItem?.type ?? 'file'"
            @confirm="handleDeleteConfirm"
        />
    </Card>
</template>
