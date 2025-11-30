<script setup lang="ts">
import { h } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Card } from "@/components/ui/card";
import { Tree, Folder, File } from "@/components/ui/file-tree";
import FileExplorerSearch from "./FileExplorerSearch.vue";
import { useFileExplorer } from "./useFileExplorer";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";
import type { FileNode } from "./types";

const { openFile } = useEditorTabs();

const {
    searchQuery,
    filteredTree,
    initialExpandedItems,
    setup,
} = useFileExplorer();

setup();

// Handle file click
async function handleFileClick(filePath: string, fileName: string) {
    try {
        // Read the file content from the backend
        const content = await invoke<string>("read_file_content", {
            relativePath: filePath,
        });

        // Open the file in a new tab
        openFile(fileName, content, filePath);
    } catch (error) {
        console.error("Failed to open file:", error);
    }
}

// Recursively render file tree nodes
function renderNode(node: FileNode): any {
    if (node.type === "folder") {
        return h(Folder, { id: node.id, name: node.name }, () =>
            node.children.map((child) => renderNode(child)),
        );
    } else {
        return h(File, {
            id: node.id,
            name: node.name,
            onClick: () => handleFileClick(node.path, node.name),
        });
    }
}
</script>

<template>
    <Card class="h-full p-2 flex flex-col gap-3">
        <!-- Search Bar -->
        <FileExplorerSearch v-model="searchQuery" />

        <!-- File Tree -->
        <div class="flex-1 overflow-auto">
            <Tree
                v-if="filteredTree.length > 0"
                initial-selected-id=""
                :initial-expanded-items="initialExpandedItems"
            >
                <component
                    v-for="node in filteredTree"
                    :key="node.id"
                    :is="renderNode(node)"
                />
            </Tree>
            <div v-else class="text-muted-foreground text-sm p-4 text-center">
                {{ searchQuery ? "No files found" : "No files" }}
            </div>
        </div>
    </Card>
</template>
