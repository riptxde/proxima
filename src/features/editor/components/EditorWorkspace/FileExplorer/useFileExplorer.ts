import { ref, onMounted, onUnmounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { FileNode } from "./types";

export function useFileExplorer() {
    const fileTree = ref<FileNode[]>([]);
    const searchQuery = ref("");
    let watchInterval: number | null = null;

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

    const setup = () => {
        onMounted(async () => {
            await initializeFileSystem();
            startWatching();
        });

        onUnmounted(() => {
            stopWatching();
        });
    };

    return {
        fileTree,
        searchQuery,
        filteredTree,
        initialExpandedItems,
        initializeFileSystem,
        loadFileTree,
        setup,
    };
}
