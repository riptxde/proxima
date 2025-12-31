import { ref, computed } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "vue-sonner";
import { useLogger } from "@/composables/useLogger";
import type { Client } from "@/types/client";

const clients = ref<Client[]>([]);
const selectedClientIds = ref<Set<string>>(new Set());
const isInitialized = ref(false);

let unlistenFn: UnlistenFn | null = null;

export function useExecutorClients() {
  const { addLog } = useLogger();

  const init = async () => {
    if (isInitialized.value) return;

    try {
      // Get initial state once
      const initialClients = await invoke<Client[]>("get_attached_clients");
      clients.value = initialClients;

      // Select all initial clients by default
      initialClients.forEach((client) => {
        selectedClientIds.value.add(client.id);
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
          selectedClientIds.value.add(client.id);
          toast.success("Client attached", {
            description: `User: ${client.username}`,
          });
          // This is logged on the backend
        });

        // Show toast for disconnected clients
        disconnectedClients.forEach((client) => {
          toast.error("Client disconnected", {
            description: `User: ${client.username}`,
          });
          // This is logged on the backend
        });

        // Update client list
        clients.value = updatedClients;

        // Clean up selected set (remove disconnected clients)
        selectedClientIds.value = new Set(
          [...selectedClientIds.value].filter((id) => updatedIds.has(id)),
        );
      });

      isInitialized.value = true;
    } catch (error) {
      addLog("error", `Failed to initialize clients: ${error}`);
    }
  };

  const cleanup = () => {
    if (unlistenFn) {
      unlistenFn();
      unlistenFn = null;
    }
  };

  const toggleClient = (id: string) => {
    if (selectedClientIds.value.has(id)) {
      selectedClientIds.value.delete(id);
    } else {
      selectedClientIds.value.add(id);
    }
    // Trigger reactivity
    selectedClientIds.value = new Set(selectedClientIds.value);
  };

  const selectAll = () => {
    clients.value.forEach((client) => {
      selectedClientIds.value.add(client.id);
    });
    selectedClientIds.value = new Set(selectedClientIds.value);
  };

  const deselectAll = () => {
    selectedClientIds.value.clear();
    selectedClientIds.value = new Set(selectedClientIds.value);
  };

  const getSelectedClientIds = () => {
    return Array.from(selectedClientIds.value);
  };

  const isClientSelected = (id: string) => {
    return selectedClientIds.value.has(id);
  };

  const selectedCount = computed(() => selectedClientIds.value.size);
  const totalCount = computed(() => clients.value.length);

  return {
    clients,
    selectedClientIds,
    isInitialized,
    selectedCount,
    totalCount,
    init,
    cleanup,
    toggleClient,
    selectAll,
    deselectAll,
    getSelectedClientIds,
    isClientSelected,
  };
}
