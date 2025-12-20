import { ref, computed } from "vue";
import type {
  Remote,
  RemoteCall,
  RemoteSpyFilters,
  RemoteDirection,
  RemoteType,
} from "../types/remote-spy";

// Raw storage of all remotes
const remotes = ref<Remote[]>([]);

// UI state
const selectedRemoteId = ref<string | null>(null);
const selectedCallId = ref<string | null>(null);
const isSpyActive = ref(false);

// Filter state
const filters = ref<RemoteSpyFilters>({
  directions: ["outgoing", "incoming"],
  types: ["RemoteEvent", "RemoteFunction"],
  search: "",
});

/**
 * Generate dummy data for testing
 * Each remote is uniquely identified by name+path+type
 */
const generateDummyData = (): Remote[] => {
  const remoteDefinitions = [
    {
      name: "PlayerDataRequest",
      path: "ReplicatedStorage.Remotes.PlayerData",
      type: "RemoteFunction" as RemoteType,
    },
    {
      name: "UpdateInventory",
      path: "ReplicatedStorage.Remotes.Inventory.Update",
      type: "RemoteEvent" as RemoteType,
    },
    {
      name: "FireWeapon",
      path: "ReplicatedStorage.Combat.WeaponFire",
      type: "RemoteEvent" as RemoteType,
    },
    {
      name: "TeleportPlayer",
      path: "ReplicatedStorage.Remotes.Teleport",
      type: "RemoteFunction" as RemoteType,
    },
    {
      name: "GetPlayerStats",
      path: "ReplicatedStorage.Remotes.Stats.Get",
      type: "RemoteFunction" as RemoteType,
    },
    {
      name: "ChatMessage",
      path: "ReplicatedStorage.Communication.Chat",
      type: "RemoteEvent" as RemoteType,
    },
  ];

  const scripts = [
    {
      name: "LocalScript",
      path: "StarterPlayer.StarterPlayerScripts.LocalScript",
    },
    {
      name: "PlayerController",
      path: "StarterPlayer.StarterPlayerScripts.Controllers.PlayerController",
    },
    {
      name: "WeaponHandler",
      path: "StarterPlayer.StarterPlayerScripts.WeaponHandler",
    },
  ];

  const dummyRemotes: Remote[] = [];

  // Generate calls for each remote
  for (const remoteDef of remoteDefinitions) {
    const callCount = Math.floor(Math.random() * 8) + 2; // 2-10 calls per remote
    const calls: RemoteCall[] = [];

    for (let i = 0; i < callCount; i++) {
      const direction: RemoteDirection =
        Math.random() > 0.5 ? "outgoing" : "incoming";
      const script = scripts[Math.floor(Math.random() * scripts.length)]!;

      // Generate arguments
      const argCount = Math.floor(Math.random() * 3) + 1;
      const args = Array.from({ length: argCount }, (_, idx) => {
        const types = ["string", "number", "boolean", "table"];
        const type = types[Math.floor(Math.random() * types.length)]!;

        let value = "";
        if (type === "string") value = `"Example ${idx}"`;
        else if (type === "number")
          value = String(Math.floor(Math.random() * 100));
        else if (type === "boolean")
          value = Math.random() > 0.5 ? "true" : "false";
        else value = "{ ... }";

        return { type, value };
      });

      const call: RemoteCall = {
        id: `${remoteDef.name}-call-${i}`,
        timestamp: new Date(Date.now() - Math.random() * 300000), // Random time in last 5 minutes
        direction,
        arguments: args,
        callingScript: script.name,
        callingScriptPath: script.path,
      };

      // Add return value for RemoteFunctions with incoming direction
      if (remoteDef.type === "RemoteFunction" && direction === "incoming") {
        call.returnValue = {
          type: Math.random() > 0.5 ? "boolean" : "table",
          value: Math.random() > 0.5 ? "true" : "{ success = true }",
        };
      }

      calls.push(call);
    }

    // Sort calls by timestamp (newest first)
    calls.sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime());

    dummyRemotes.push({
      id: `${remoteDef.name}|${remoteDef.path}|${remoteDef.type}`,
      name: remoteDef.name,
      path: remoteDef.path,
      type: remoteDef.type,
      calls,
    });
  }

  return dummyRemotes;
};

export function useRemoteSpy() {
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
  const selectRemote = (id: string) => {
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
  const selectCall = (id: string) => {
    selectedCallId.value = id;
  };

  /**
   * Clear all calls
   */
  const clearCalls = () => {
    remotes.value = [];
    selectedRemoteId.value = null;
    selectedCallId.value = null;
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

  /**
   * Start the spy (generates dummy data for now)
   */
  const startSpy = () => {
    isSpyActive.value = true;
    remotes.value = generateDummyData();
  };

  /**
   * Stop the spy
   */
  const stopSpy = () => {
    isSpyActive.value = false;
  };

  /**
   * Generate Lua code for a remote call
   */
  const generateCodeForCall = (remote: Remote, call: RemoteCall): string => {
    const args = call.arguments.map((arg) => arg.value).join(", ");
    const pathParts = remote.path.split(".");
    const service = pathParts[0];
    const path = pathParts.slice(1).join(".");

    if (remote.type === "RemoteEvent") {
      const method =
        call.direction === "outgoing" ? "FireServer" : "FireClient";
      const target = call.direction === "outgoing" ? "" : "player, ";
      return `game:GetService("${service}").${path}:${method}(${target}${args})`;
    } else {
      const method =
        call.direction === "outgoing" ? "InvokeServer" : "InvokeClient";
      const target = call.direction === "outgoing" ? "" : "player, ";
      return `local result = game:GetService("${service}").${path}:${method}(${target}${args})`;
    }
  };

  return {
    // State
    remotes: filteredRemotes,
    selectedRemote,
    selectedCall,
    selectedRemoteId,
    selectedCallId,
    isSpyActive,
    filters,

    // Actions
    selectRemote,
    deselectRemote,
    selectCall,
    clearCalls,
    toggleDirectionFilter,
    toggleTypeFilter,
    setSearchFilter,
    startSpy,
    stopSpy,
    generateCodeForCall,

    // Helpers
    getDirectionCount,
    getTypeCount,
  };
}
