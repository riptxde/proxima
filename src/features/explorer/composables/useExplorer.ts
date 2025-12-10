import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  ExplorerItem,
  ExplorerClient,
  ExplorerProperty,
  ExplorerSearchResult,
} from "../types/explorer";

// State
const selectedClient = ref<ExplorerClient | null>(null);
const explorerItems = ref<ExplorerItem[]>([]);
const availableClients = ref<ExplorerClient[]>([]);
const isExplorerActive = ref(false);
const selectedItemId = ref<string | null>(null);
const selectedItemName = ref<string | null>(null);
const selectedItemProperties = ref<ExplorerProperty[]>([]);
const expandedIds = ref<Set<string>>(new Set());
const searchResults = ref<ExplorerSearchResult[]>([]);
const searchQuery = ref<string>("");
const searchLimited = ref<boolean>(false);

// Event listeners
let unlistenTree: UnlistenFn | null = null;
let unlistenProperties: UnlistenFn | null = null;
let unlistenSearchResults: UnlistenFn | null = null;
let unlistenTreeChanged: UnlistenFn | null = null;
let unlistenExplorerStarted: UnlistenFn | null = null;
let unlistenExplorerStopped: UnlistenFn | null = null;
let unlistenClientsUpdate: UnlistenFn | null = null;

export function useExplorer() {
  // Commands
  const startExplorer = async (client: ExplorerClient) => {
    try {
      await invoke("start_explorer", { clientId: client.id });
      selectedClient.value = client;
      isExplorerActive.value = true;
    } catch (error) {
      console.error("Failed to start explorer:", error);
      throw error;
    }
  };

  const stopExplorer = async () => {
    try {
      await invoke("stop_explorer");
      resetExplorerState();
    } catch (error) {
      console.error("Failed to stop explorer:", error);
      throw error;
    }
  };

  const getTree = async (ids: string[]) => {
    try {
      const numericIds = ids.map((id) => parseInt(id, 10));
      await invoke("explorer_get_tree", { expandedIds: numericIds });
    } catch (error) {
      console.error("Failed to get tree:", error);
      throw error;
    }
  };

  const getProperties = async (id: string, className: string, name: string) => {
    try {
      selectedItemId.value = id;
      selectedItemName.value = name;

      await invoke("explorer_get_properties", {
        id: parseInt(id, 10),
        className,
      });
    } catch (error) {
      console.error("Failed to get properties:", error);
      throw error;
    }
  };

  const search = async (query: string, searchBy: string, limit: number) => {
    try {
      await invoke("explorer_search", { query, searchBy, limit });
    } catch (error) {
      console.error("Failed to search:", error);
      throw error;
    }
  };

  const toggleExpand = (itemId: string) => {
    if (expandedIds.value.has(itemId)) {
      expandedIds.value.delete(itemId);
    } else {
      expandedIds.value.add(itemId);
    }
    getTree(Array.from(expandedIds.value));
  };

  const resetExplorerState = () => {
    selectedClient.value = null;
    explorerItems.value = [];
    isExplorerActive.value = false;
    selectedItemId.value = null;
    selectedItemName.value = null;
    selectedItemProperties.value = [];
    expandedIds.value.clear();
  };

  // Event listeners
  const initializeListeners = async () => {
    unlistenTree = await listen<{ nodes: any[] }>("explorer-tree", (event) => {
      explorerItems.value = convertNodesToExplorerItems(event.payload.nodes);
    });

    unlistenProperties = await listen<{
      id: number;
      props: Record<string, any>;
      specialProps: Record<string, any>;
    }>("explorer-properties", (event) => {
      if (selectedItemId.value === event.payload.id.toString()) {
        selectedItemProperties.value = convertPropertiesToArray(
          event.payload.props,
          event.payload.specialProps,
        );
      }
    });

    unlistenSearchResults = await listen<{
      query: string;
      results: any[];
      total: number;
      limited: boolean;
    }>("explorer-search-results", (event) => {
      searchQuery.value = event.payload.query;
      searchLimited.value = event.payload.limited;
      searchResults.value = event.payload.results.map((result: any) => ({
        id: result.id.toString(),
        name: result.n,
        className: result.c,
        path: Array.isArray(result.p) ? result.p : [],
        pathString: result.s || "game",
      }));
    });

    unlistenTreeChanged = await listen("explorer-tree-changed", () => {
      getTree(Array.from(expandedIds.value));
    });

    unlistenExplorerStarted = await listen("explorer-started", () => {
      isExplorerActive.value = true;
    });

    unlistenExplorerStopped = await listen("explorer-stopped", () => {
      resetExplorerState();
    });

    unlistenClientsUpdate = await listen<ExplorerClient[]>(
      "clients-update",
      (event) => {
        availableClients.value = event.payload;
      },
    );
  };

  const cleanupListeners = () => {
    unlistenTree?.();
    unlistenProperties?.();
    unlistenSearchResults?.();
    unlistenTreeChanged?.();
    unlistenExplorerStarted?.();
    unlistenExplorerStopped?.();
    unlistenClientsUpdate?.();
  };

  return {
    // State
    selectedClient,
    availableClients,
    explorerItems,
    isExplorerActive,
    selectedItemId,
    selectedItemName,
    selectedItemProperties,
    expandedIds: computed(() => expandedIds.value),
    searchResults,
    searchQuery,
    searchLimited,
    // Commands
    startExplorer,
    stopExplorer,
    getTree,
    getProperties,
    search,
    toggleExpand,
    // Listeners
    initializeListeners,
    cleanupListeners,
  };
}

// Helper functions
function convertNodesToExplorerItems(nodes: any[]): ExplorerItem[] {
  return nodes.map((node) => ({
    id: node.id.toString(),
    name: node.n,
    className: node.c,
    hasChildren: node.h,
    children: node.children ? convertNodesToExplorerItems(node.children) : [],
  }));
}

function convertPropertiesToArray(
  props: Record<string, any>,
  specialProps: Record<string, any>,
): ExplorerProperty[] {
  const properties: ExplorerProperty[] = [];

  for (const [name, data] of Object.entries(props)) {
    properties.push({
      name,
      type: data.type || "unknown",
      value: data.value || "",
      readOnly: data.hidden || data.notScriptable || false,
      deprecated: data.deprecated || false,
      hidden: data.hidden || false,
      notScriptable: data.notScriptable || false,
    });
  }

  for (const [name, data] of Object.entries(specialProps)) {
    properties.push({
      name,
      type: data.type || "unknown",
      value: data.value || "",
      readOnly: true,
      deprecated: data.deprecated || false,
      hidden: data.hidden || false,
      notScriptable: data.notScriptable || false,
    });
  }

  return properties;
}
