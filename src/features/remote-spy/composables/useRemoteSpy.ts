import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useLogger } from "@/composables/useLogger";
import type {
  Remote,
  RemoteCall,
  RemoteSpyFilters,
  RemoteDirection,
  RemoteType,
  RemoteSpyClient,
} from "../types/remote-spy";

// State
const remotes = ref<Remote[]>([]);
const selectedRemoteId = ref<number | null>(null);
const selectedCallId = ref<number | null>(null);
const isSpyActive = ref(false);
const isPaused = ref(false);
const selectedClient = ref<RemoteSpyClient | null>(null);
const availableClients = ref<RemoteSpyClient[]>([]);

// Filter state
const filters = ref<RemoteSpyFilters>({
  directions: ["outgoing", "incoming"],
  types: ["RemoteEvent", "RemoteFunction", "UnreliableRemoteEvent"],
  search: "",
});

export function useRemoteSpy() {
  const { addLog } = useLogger();

  /**
   * Filtered remotes based on search and type filters
   */
  const filteredRemotes = computed(() => {
    return remotes.value
      .filter((remote) => {
        // Filter by type
        if (!filters.value.types.includes(remote.type)) return false;

        // Filter by search
        if (filters.value.search) {
          const searchLower = filters.value.search.toLowerCase();
          const matchesName = remote.name.toLowerCase().includes(searchLower);
          const matchesPath = remote.path.toLowerCase().includes(searchLower);
          if (!matchesName && !matchesPath) return false;
        }

        // Filter by direction (only show remotes that have calls matching the direction filter)
        const hasMatchingCalls = remote.calls.some((call) =>
          filters.value.directions.includes(call.direction),
        );
        if (!hasMatchingCalls) return false;

        return true;
      })
      .map((remote) => ({
        ...remote,
        // Filter calls by direction
        calls: remote.calls.filter((call) =>
          filters.value.directions.includes(call.direction),
        ),
      }))
      .filter((remote) => remote.calls.length > 0) // Remove remotes with no matching calls
      .sort((a, b) => {
        // Sort by latest call timestamp
        const aLatest = Math.max(...a.calls.map((c) => c.timestamp.getTime()));
        const bLatest = Math.max(...b.calls.map((c) => c.timestamp.getTime()));
        return bLatest - aLatest;
      });
  });

  /**
   * Selected remote object
   */
  const selectedRemote = computed(() => {
    if (!selectedRemoteId.value) return null;
    return (
      filteredRemotes.value.find(
        (remote) => remote.id === selectedRemoteId.value,
      ) || null
    );
  });

  /**
   * Selected call object
   */
  const selectedCall = computed(() => {
    if (!selectedRemoteId.value || !selectedCallId.value) return null;
    const remote = selectedRemote.value;
    if (!remote) return null;
    return (
      remote.calls.find((call) => call.id === selectedCallId.value) || null
    );
  });

  /**
   * Get count of calls by direction across all remotes
   */
  const getDirectionCount = (direction: RemoteDirection) => {
    return remotes.value.reduce((count, remote) => {
      return (
        count +
        remote.calls.filter((call) => call.direction === direction).length
      );
    }, 0);
  };

  /**
   * Get count of remotes by type
   */
  const getTypeCount = (type: RemoteType) => {
    return remotes.value.filter((remote) => remote.type === type).length;
  };

  /**
   * Select a remote
   */
  const selectRemote = (id: number) => {
    selectedRemoteId.value = id;
    selectedCallId.value = null; // Clear call selection
  };

  /**
   * Deselect remote
   */
  const deselectRemote = () => {
    selectedRemoteId.value = null;
    selectedCallId.value = null;
  };

  /**
   * Select a call
   */
  const selectCall = (id: number) => {
    selectedCallId.value = id;
  };

  /**
   * Clear all calls
   * @returns The number of calls cleared
   */
  const clearCalls = (): number => {
    const totalCalls = remotes.value.reduce(
      (sum, remote) => sum + remote.calls.length,
      0,
    );
    remotes.value = [];
    selectedRemoteId.value = null;
    selectedCallId.value = null;
    return totalCalls;
  };

  /**
   * Toggle direction filter
   */
  const toggleDirectionFilter = (direction: RemoteDirection) => {
    const index = filters.value.directions.indexOf(direction);
    if (index === -1) {
      filters.value.directions.push(direction);
    } else if (filters.value.directions.length > 1) {
      filters.value.directions.splice(index, 1);
    }
  };

  /**
   * Toggle type filter
   */
  const toggleTypeFilter = (type: RemoteType) => {
    const index = filters.value.types.indexOf(type);
    if (index === -1) {
      filters.value.types.push(type);
    } else if (filters.value.types.length > 1) {
      filters.value.types.splice(index, 1);
    }
  };

  /**
   * Set search filter
   */
  const setSearchFilter = (search: string) => {
    filters.value.search = search;
  };

  // Commands
  const rspyStart = async (client: RemoteSpyClient) => {
    try {
      await invoke("rspy_start", { clientId: client.id });
      selectedClient.value = client;
      isSpyActive.value = true;
      isPaused.value = false;
    } catch (error) {
      addLog("error", `Failed to start remote spy: ${error}`);
      throw error;
    }
  };

  const togglePause = () => {
    isPaused.value = !isPaused.value;
  };

  const rspyStop = async () => {
    try {
      await invoke("rspy_stop");
      resetRemoteSpyState();
    } catch (error) {
      addLog("error", `Failed to stop remote spy: ${error}`);
      throw error;
    }
  };

  const rspyDecompile = async (scriptPath: string) => {
    try {
      await invoke("rspy_decompile", { scriptPath });
    } catch (error) {
      addLog("error", `Failed to decompile script: ${error}`);
      throw error;
    }
  };

  const rspyGenerateCode = async (callId: number) => {
    try {
      await invoke("rspy_generate_code", { callId });
    } catch (error) {
      addLog("error", `Failed to generate code: ${error}`);
      throw error;
    }
  };

  const resetRemoteSpyState = () => {
    selectedClient.value = null;
    isSpyActive.value = false;
    isPaused.value = false;
  };

  // Event listeners
  const initializeListeners = async () => {
    await listen<any>("remote-spy-call", (event) => {
      // If paused, don't add new calls to the UI
      if (isPaused.value) {
        return;
      }

      const callData = event.payload;

      // Find or create remote
      let remote = remotes.value.find((r) => r.id === callData.remoteId);

      if (!remote) {
        remote = {
          id: callData.remoteId,
          name: callData.name,
          path: callData.path,
          type: callData.class as RemoteType,
          calls: [],
        };
        remotes.value.push(remote);
      }

      // Create call object
      const call: RemoteCall = {
        id: callData.callId,
        timestamp: new Date(callData.timestamp),
        direction: callData.direction as RemoteDirection,
        arguments: callData.arguments || [],
        returnValue: callData.returnValue,
        callingScriptName: callData.callingScriptName,
        callingScriptPath: callData.callingScriptPath,
      };

      // Add call to remote (prepend for newest first)
      remote.calls.unshift(call);
    });

    await listen("remote-spy-started", () => {
      isSpyActive.value = true;
    });

    await listen("remote-spy-stopped", () => {
      resetRemoteSpyState();
    });

    await listen<{
      scriptPath: string;
      source: string;
    }>("remote-spy-decompiled", (event) => {
      // Emit a custom event that components can listen to
      window.dispatchEvent(
        new CustomEvent("remote-spy-decompiled", {
          detail: event.payload,
        }),
      );
    });

    await listen<{
      callId: number;
      code: string;
    }>("remote-spy-generated-code", (event) => {
      // Store the generated code temporarily
      // Emit a custom event that components can listen to
      window.dispatchEvent(
        new CustomEvent("remote-spy-code-generated", {
          detail: event.payload,
        }),
      );
    });
  };

  // Initialize remote spy client listeners
  const initializeRemoteSpyClientListeners = async () => {
    await listen<RemoteSpyClient[]>("clients-update", (event) => {
      availableClients.value = event.payload;

      // If the selected client is no longer available, reset remote spy state
      if (selectedClient.value) {
        const clientExists = event.payload.some(
          (client) => client.id === selectedClient.value?.id,
        );

        if (!clientExists && isSpyActive.value) {
          resetRemoteSpyState();
        }
      }
    });
  };

  return {
    // State
    remotes: filteredRemotes,
    selectedRemote,
    selectedCall,
    selectedRemoteId,
    selectedCallId,
    isSpyActive,
    isPaused,
    selectedClient,
    availableClients,
    filters,

    // Actions
    selectRemote,
    deselectRemote,
    selectCall,
    clearCalls,
    toggleDirectionFilter,
    toggleTypeFilter,
    setSearchFilter,
    startSpy: rspyStart,
    stopSpy: rspyStop,
    togglePause,
    decompileScript: rspyDecompile,
    generateCode: rspyGenerateCode,

    // Listeners
    initializeListeners,
    initializeRemoteSpyClientListeners,

    // Helpers
    getDirectionCount,
    getTypeCount,
  };
}
