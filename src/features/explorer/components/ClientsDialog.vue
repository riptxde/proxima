<script setup lang="ts">
import { ref, computed } from "vue";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Search } from "lucide-vue-next";
import { toast } from "vue-sonner";
import { useExplorer } from "../composables/useExplorer";
import type { ExplorerClient } from "../types/explorer";

defineProps<{
  open: boolean;
}>();

const emit = defineEmits<{
  "update:open": [value: boolean];
}>();

const { selectedClient, availableClients, startExplorer, getTree } =
  useExplorer();
const searchQuery = ref("");

const filteredClients = computed(() => {
  if (!searchQuery.value.trim()) {
    return availableClients.value;
  }

  const query = searchQuery.value.toLowerCase();
  return availableClients.value.filter((client) =>
    client.username.toLowerCase().includes(query),
  );
});

const selectClient = async (client: ExplorerClient) => {
  try {
    await startExplorer(client);
    await getTree([]);
    toast.success(`Explorer connected to ${client.username}`);
    emit("update:open", false);
  } catch (error) {
    toast.error(`Failed to start explorer: ${error}`);
  }
};
</script>

<template>
  <Dialog :open="open" @update:open="$emit('update:open', $event)">
    <DialogContent class="sm:max-w-[500px]">
      <DialogHeader>
        <DialogTitle>Select Client</DialogTitle>
        <DialogDescription>
          Choose a client to view its explorer tree.
        </DialogDescription>
      </DialogHeader>

      <div class="space-y-4">
        <div class="relative">
          <Search
            class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground"
          />
          <Input v-model="searchQuery" placeholder="Search" class="pl-9" />
        </div>

        <div
          v-if="availableClients.length === 0"
          class="text-center py-12 text-muted-foreground"
        >
          <p class="text-sm">No clients attached</p>
        </div>

        <div
          v-else-if="filteredClients.length === 0"
          class="text-center py-12 text-muted-foreground"
        >
          <p class="text-sm">No clients match your search</p>
        </div>

        <div v-else class="rounded-md border overflow-hidden">
          <div class="max-h-[50vh] overflow-y-auto">
            <div
              v-for="client in filteredClients"
              :key="client.id"
              class="px-4 py-2 hover:bg-accent/50 cursor-pointer transition-colors border-b last:border-b-0"
              :class="{
                'bg-accent/30': selectedClient?.id === client.id,
              }"
              @click="selectClient(client)"
            >
              <div class="flex-1 min-w-0">
                <div class="text-sm">{{ client.username }}</div>
                <div class="text-xs text-muted-foreground font-mono">
                  {{ client.id }}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
