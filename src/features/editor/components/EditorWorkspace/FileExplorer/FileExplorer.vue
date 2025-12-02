<script setup lang="ts">
import { h } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Card } from "@/components/ui/card";
import { Tree, Folder, File } from "@/components/ui/file-tree";
import { Spinner } from "@/components/ui/spinner";
import FileExplorerSearch from "./FileExplorerSearch.vue";
import { useFileExplorer } from "./useFileExplorer";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";
import type { FileNode } from "./types";

const { openFile } = useEditorTabs();

const {
    searchQuery,
    debouncedSearchQuery,
    filteredTree,
    expandedItems,
    isLoading,
    isSearching,
    resultsLimited,
} = useFileExplorer();

async function handleFileClick(filePath: string, fileName: string) {
    try {
        const content = await invoke<string>("read_file_content", {
            relativePath: filePath,
        });
        openFile(fileName, content, filePath);
    } catch (error) {
        const errorMessage =
            error instanceof Error ? error.message : String(error);
        invoke("add_log", {
            level: 3,
            message: `Failed to open file ${fileName}: ${errorMessage}`,
        }).catch(() => {});
    }
}

function renderNode(node: FileNode): any {
    if (node.type === "folder") {
        return h(Folder, { id: node.id, name: node.name }, () =>
            node.children.map((child) => renderNode(child)),
        );
    }
    return h(File, {
        id: node.id,
        name: node.name,
        onClick: () => handleFileClick(node.path, node.name),
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
    </Card>
</template>
