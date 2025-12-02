import { ref, computed } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "vue-sonner";
import type { Client } from "../types/executor";

const clients = ref<Client[]>([]);
const enabledClientIds = ref<Set<string>>(new Set());
const isInitialized = ref(false);

let unlistenFn: UnlistenFn | null = null;

export function useClients() {
  const initialize = async () => {
    if (isInitialized.value) return;

    try {
      // Get initial state once
      const initialClients = await invoke<Client[]>("get_connected_clients");
      clients.value = initialClients;

      // Enable all initial clients by default
      initialClients.forEach((client) => {
        enabledClientIds.value.add(client.id);
      });

      // Listen for all future updates
      unlistenFn = await listen<Client[]>("clients-update", (event) => {
        const updatedClients = event.payload;

        // Find new clients (in updated but not in current)
        const currentIds = new Set(clients.value.map((c) => c.id));
        const newClients = updatedClients.filter((c) => !currentIds.has(c.id));

        // Find disconnected clients (in current but not in updated)
        const updatedIds = new Set(updatedClients.map((c) => c.id));
        const disconnectedClients = clients.value.filter(
          (c) => !updatedIds.has(c.id),
        );

        // Show toast for new clients
        newClients.forEach((client) => {
          enabledClientIds.value.add(client.id);
          toast.success("Client attached", {
            description: client.username,
          });
          // This is logged on the backend
        });

        // Show toast for disconnected clients
        disconnectedClients.forEach((client) => {
          toast.error("Client disconnected", {
            description: client.username,
          });
          // This is logged on the backend
        });

        // Update client list
        clients.value = updatedClients;

        // Clean up enabled set (remove disconnected clients)
        enabledClientIds.value = new Set(
          [...enabledClientIds.value].filter((id) => updatedIds.has(id)),
        );
      });

      isInitialized.value = true;
    } catch (error) {
      console.error(error);
    }
  };

  const cleanup = () => {
    if (unlistenFn) {
      unlistenFn();
      unlistenFn = null;
    }
  };

  const toggleClient = (id: string) => {
    if (enabledClientIds.value.has(id)) {
      enabledClientIds.value.delete(id);
    } else {
      enabledClientIds.value.add(id);
    }
    // Trigger reactivity
    enabledClientIds.value = new Set(enabledClientIds.value);
  };

  const enableAll = () => {
    clients.value.forEach((client) => {
      enabledClientIds.value.add(client.id);
    });
    enabledClientIds.value = new Set(enabledClientIds.value);
  };

  const disableAll = () => {
    enabledClientIds.value.clear();
    enabledClientIds.value = new Set(enabledClientIds.value);
  };

  const getEnabledClientIds = () => {
    return Array.from(enabledClientIds.value);
  };

  const isClientEnabled = (id: string) => {
    return enabledClientIds.value.has(id);
  };

  const enabledCount = computed(() => enabledClientIds.value.size);
  const totalCount = computed(() => clients.value.length);

  return {
    clients,
    enabledClientIds,
    isInitialized,
    enabledCount,
    totalCount,
    initialize,
    cleanup,
    toggleClient,
    enableAll,
    disableAll,
    getEnabledClientIds,
    isClientEnabled,
  };
}
