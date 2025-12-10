<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import ExplorerItem from "./ExplorerItem.vue";
import ExplorerDock from "./ExplorerDock.vue";
import { useExplorer } from "../composables/useExplorer";
import type { ExplorerProperty } from "../types/explorer";

const {
  selectedClient,
  explorerItems,
  availableClients,
  selectedItemId,
  selectedItemProperties,
  initializeListeners,
  cleanupListeners,
  getTree,
} = useExplorer();

const selectedProperty = ref<ExplorerProperty | null>(null);

onMounted(async () => {
  await initializeListeners();

  // Fetch initial client list
  try {
    const clients = await invoke("get_attached_clients");
    availableClients.value = clients as any[];
  } catch (error) {
    console.error("Failed to get clients:", error);
  }

  // Request initial tree if explorer is active
  if (selectedClient.value) {
    getTree([]);
  }
});

onUnmounted(() => {
  cleanupListeners();
});

const handleSelectProperty = (property: ExplorerProperty) => {
  selectedProperty.value = property;
};
</script>

<template>
  <div
    class="h-full overflow-hidden flex flex-col rounded-lg border border-border shadow-sm"
  >
    <div class="flex-1 overflow-hidden pt-4 px-4">
      <div class="h-full">
        <ResizablePanelGroup direction="horizontal" class="h-full gap-4">
          <!-- Explorer Tree -->
          <ResizablePanel :default-size="40" :min-size="25">
            <div
              class="h-full flex flex-col bg-card/50 rounded-lg border border-border/50"
            >
              <div v-if="selectedClient" class="flex-1 overflow-y-auto p-2">
                <ExplorerItem
                  v-for="item in explorerItems"
                  :key="item.id"
                  :item="item"
                />
              </div>
              <div
                v-else
                class="flex-1 flex items-center justify-center text-muted-foreground text-sm"
              >
                Select a client to view explorer
              </div>
            </div>
          </ResizablePanel>

          <ResizableHandle with-handle />

          <!-- Properties Panel -->
          <ResizablePanel :default-size="60" :min-size="25">
            <div
              class="h-full flex flex-col bg-card/50 rounded-lg border border-border/50"
            >
              <div v-if="selectedClient" class="flex-1 overflow-y-auto">
                <div v-if="selectedItemId" class="divide-y divide-border/30">
                  <div
                    v-for="property in selectedItemProperties"
                    :key="property.name"
                    class="px-4 py-2 hover:bg-accent/20 cursor-pointer transition-colors"
                    :class="{
                      'bg-accent/30': selectedProperty?.name === property.name,
                    }"
                    @click="handleSelectProperty(property)"
                  >
                    <div class="flex items-start justify-between gap-2">
                      <div class="flex-1 min-w-0">
                        <div class="flex items-center gap-2">
                          <span class="text-sm font-medium truncate">{{
                            property.name
                          }}</span>
                          <span
                            class="text-xs px-1.5 py-0.5 rounded bg-muted/50 text-muted-foreground"
                            >{{ property.type }}</span
                          >
                        </div>
                        <div
                          class="text-sm text-muted-foreground mt-1 truncate"
                        >
                          {{ property.value }}
                        </div>
                      </div>
                      <div v-if="property.readOnly" class="shrink-0">
                        <span
                          class="text-xs px-1.5 py-0.5 rounded bg-yellow-500/20 text-yellow-500"
                          >readonly</span
                        >
                      </div>
                    </div>
                  </div>
                </div>
                <div
                  v-else
                  class="flex-1 flex items-center justify-center text-muted-foreground text-sm h-full"
                >
                  Select an item to view its properties
                </div>
              </div>
              <div
                v-else
                class="flex-1 flex items-center justify-center text-muted-foreground text-sm"
              >
                Select a client to view properties
              </div>
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </div>

    <ExplorerDock />
  </div>
</template>
