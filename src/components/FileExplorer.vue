<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, h } from "vue";
import { Tree, Folder, File } from "@/components/ui/file-tree";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Search } from "lucide-vue-next";
import { invoke } from "@tauri-apps/api/core";
import type { FileNode } from "@/types/fileTree";
import { useTabState } from "@/composables/useTabState";

const fileTree = ref<FileNode[]>([]);
const searchQuery = ref("");
let watchInterval: number | null = null;

const { openFile } = useTabState();

// Initialize directories and load file tree
async function initializeFileSystem() {
    try {
        // Create directories if they don't exist
        await invoke("initialize_directories");

        // Load the file tree
        await loadFileTree();
    } catch (error) {
        console.error("Failed to initialize file system:", error);
    }
}

async function loadFileTree() {
    try {
        const tree = await invoke<FileNode[]>("read_file_tree");
        fileTree.value = tree;
    } catch (error) {
        console.error("Failed to load file tree:", error);
    }
}

// Watch for file system changes by polling
function startWatching() {
    // Poll every 2 seconds for file system changes
    watchInterval = window.setInterval(async () => {
        await loadFileTree();
    }, 2000);
}

function stopWatching() {
    if (watchInterval !== null) {
        clearInterval(watchInterval);
        watchInterval = null;
    }
}

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

// Filter file tree based on search query
const filteredTree = computed(() => {
    if (!searchQuery.value) {
        return fileTree.value;
    }

    const query = searchQuery.value.toLowerCase();

    function filterNode(node: FileNode): FileNode | null {
        if (node.type === "file") {
            return node.name.toLowerCase().includes(query) ? node : null;
        } else {
            const filteredChildren = node.children
                .map((child) => filterNode(child))
                .filter((child) => child !== null) as FileNode[];

            if (
                filteredChildren.length > 0 ||
                node.name.toLowerCase().includes(query)
            ) {
                return {
                    ...node,
                    children: filteredChildren,
                };
            }
            return null;
        }
    }

    return fileTree.value
        .map((node) => filterNode(node))
        .filter((node) => node !== null) as FileNode[];
});

// Get initial expanded items
const initialExpandedItems = computed(() => {
    return fileTree.value.map((node) => node.id);
});

onMounted(async () => {
    await initializeFileSystem();
    startWatching();
});

onUnmounted(() => {
    stopWatching();
});
</script>

<template>
    <Card class="h-full p-2 flex flex-col gap-3">
        <!-- Search Bar -->
        <div class="relative">
            <Search
                class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground"
            />
            <Input
                v-model="searchQuery"
                type="text"
                placeholder="Search"
                class="pl-9 border-none h-[46px] text-sm bg-tab-bar"
            />
        </div>

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
