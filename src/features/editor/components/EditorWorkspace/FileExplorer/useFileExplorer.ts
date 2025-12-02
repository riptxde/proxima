import { ref, onMounted, onUnmounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { watchDebounced } from "@vueuse/core";
import type { FileNode } from "./types";

const MAX_SEARCH_RESULTS = 100;
const SEARCH_DEBOUNCE_MS = 300;

export function useFileExplorer() {
  const fileTree = ref<FileNode[]>([]);
  const searchQuery = ref("");
  const debouncedSearchQuery = ref("");
  const isLoading = ref(true);
  const isSearching = ref(false);
  const resultsLimited = ref(false);
  let unlistenFn: UnlistenFn | null = null;

  async function loadFileTree() {
    try {
      const tree = await invoke<FileNode[]>("read_file_tree");
      fileTree.value = tree;
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : String(error);
      invoke("add_log", {
        level: 3,
        message: `Failed to load file tree: ${errorMessage}`,
      }).catch(() => {});
    }
  }

  // Debounced search with loading state management
  watchDebounced(
    searchQuery,
    (newValue) => {
      debouncedSearchQuery.value = newValue;
      isSearching.value = false;
    },
    {
      debounce: SEARCH_DEBOUNCE_MS,
      onTrigger: () => {
        // Show searching indicator when user types
        if (searchQuery.value !== debouncedSearchQuery.value) {
          isSearching.value = true;
        }
      },
    },
  );

  // Filter file tree based on debounced search query
  const filteredTree = computed(() => {
    if (!debouncedSearchQuery.value) {
      resultsLimited.value = false;
      return fileTree.value;
    }

    const query = debouncedSearchQuery.value.toLowerCase();
    let matchCount = 0;

    function filterNode(node: FileNode): FileNode | null {
      if (matchCount >= MAX_SEARCH_RESULTS) {
        return null;
      }

      if (node.type === "file") {
        if (node.name.toLowerCase().includes(query)) {
          matchCount++;
          return node;
        }
        return null;
      }

      const filteredChildren = node.children
        .map((child) => filterNode(child))
        .filter((child): child is FileNode => child !== null);

      if (
        filteredChildren.length > 0 ||
        node.name.toLowerCase().includes(query)
      ) {
        return { ...node, children: filteredChildren };
      }

      return null;
    }

    const result = fileTree.value
      .map((node) => filterNode(node))
      .filter((node): node is FileNode => node !== null);

    resultsLimited.value = matchCount >= MAX_SEARCH_RESULTS;
    return result;
  });

  // Expand all folders when searching to show matches
  const expandedItems = computed(() => {
    const tree = debouncedSearchQuery.value
      ? filteredTree.value
      : fileTree.value;
    const expandedIds: string[] = [];

    function collectFolderIds(node: FileNode) {
      if (node.type === "folder") {
        expandedIds.push(node.id);
        // When searching, expand all folders; otherwise only expand root
        if (debouncedSearchQuery.value) {
          node.children.forEach(collectFolderIds);
        }
      }
    }

    tree.forEach(collectFolderIds);
    return expandedIds;
  });

  onMounted(async () => {
    try {
      // Initialize directories and load file tree
      await invoke("initialize_directories");
      await loadFileTree();

      // Start watching for file changes
      unlistenFn = await listen("file-tree-changed", loadFileTree);
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : String(error);
      invoke("add_log", {
        level: 3,
        message: `Failed to initialize file system: ${errorMessage}`,
      }).catch(() => {});
    } finally {
      isLoading.value = false;
    }
  });

  onUnmounted(() => {
    unlistenFn?.();
  });

  return {
    searchQuery,
    debouncedSearchQuery,
    filteredTree,
    expandedItems,
    isLoading,
    isSearching,
    resultsLimited,
  };
}
