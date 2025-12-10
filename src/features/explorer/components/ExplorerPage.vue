<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
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

// Separate normal and special properties
const normalProperties = computed(() => {
  return selectedItemProperties.value
    .filter((p) => !p.hidden && !p.notScriptable)
    .sort((a, b) => a.name.localeCompare(b.name));
});

const specialProperties = computed(() => {
  return selectedItemProperties.value
    .filter((p) => p.hidden || p.notScriptable)
    .sort((a, b) => a.name.localeCompare(b.name));
});

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
                <div v-if="selectedItemId" class="p-4 space-y-4">
                  <!-- Normal Properties Section -->
                  <div
                    v-if="
                      normalProperties.length > 0 ||
                      specialProperties.length === 0
                    "
                  >
                    <h3
                      class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3"
                    >
                      Properties
                    </h3>
                    <div class="space-y-2">
                      <div
                        v-for="property in normalProperties"
                        :key="property.name"
                        class="p-3 rounded-lg border border-border/50 hover:bg-accent/10 cursor-pointer transition-colors"
                        :class="{
                          'bg-accent/20 border-accent/50':
                            selectedProperty?.name === property.name,
                        }"
                        @click="handleSelectProperty(property)"
                      >
                        <div
                          class="flex items-start justify-between gap-2 mb-2"
                        >
                          <div class="flex items-center gap-2 flex-wrap">
                            <span class="text-sm font-medium">{{
                              property.name
                            }}</span>
                            <span
                              class="text-xs px-1.5 py-0.5 rounded bg-muted/50 text-muted-foreground font-mono"
                              >{{
                                property.type === "className"
                                  ? "ClassName"
                                  : property.type
                              }}</span
                            >
                          </div>
                          <div class="flex items-center gap-1.5 shrink-0">
                            <span
                              v-if="property.deprecated"
                              class="text-xs px-1.5 py-0.5 rounded bg-orange-500/20 text-orange-400 font-medium"
                              >Deprecated</span
                            >
                            <span
                              v-if="property.readOnly"
                              class="text-xs px-1.5 py-0.5 rounded bg-yellow-500/20 text-yellow-400 font-medium"
                              >Readonly</span
                            >
                          </div>
                        </div>
                        <div
                          class="text-sm px-2 py-1.5 rounded bg-muted/30 font-mono break-all"
                        >
                          {{ property.value }}
                        </div>
                      </div>
                    </div>
                  </div>

                  <!-- Hidden & Unscriptable Properties Section -->
                  <div v-if="specialProperties.length > 0">
                    <h3
                      class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3"
                    >
                      Hidden & Unscriptable Properties
                    </h3>
                    <div class="space-y-2">
                      <div
                        v-for="property in specialProperties"
                        :key="property.name"
                        class="p-3 rounded-lg border border-border/50 hover:bg-accent/10 cursor-pointer transition-colors"
                        :class="{
                          'bg-accent/20 border-accent/50':
                            selectedProperty?.name === property.name,
                        }"
                        @click="handleSelectProperty(property)"
                      >
                        <div
                          class="flex items-start justify-between gap-2 mb-2"
                        >
                          <div class="flex items-center gap-2 flex-wrap">
                            <span class="text-sm font-medium">{{
                              property.name
                            }}</span>
                            <span
                              class="text-xs px-1.5 py-0.5 rounded bg-muted/50 text-muted-foreground font-mono"
                              >{{
                                property.type === "className"
                                  ? "ClassName"
                                  : property.type
                              }}</span
                            >
                          </div>
                          <div
                            class="flex items-center gap-1.5 shrink-0 flex-wrap"
                          >
                            <span
                              v-if="property.hidden"
                              class="text-xs px-1.5 py-0.5 rounded bg-purple-500/20 text-purple-400 font-medium"
                              >Hidden</span
                            >
                            <span
                              v-if="property.notScriptable"
                              class="text-xs px-1.5 py-0.5 rounded bg-blue-500/20 text-blue-400 font-medium"
                              >Unscriptable</span
                            >
                            <span
                              v-if="property.deprecated"
                              class="text-xs px-1.5 py-0.5 rounded bg-orange-500/20 text-orange-400 font-medium"
                              >Deprecated</span
                            >
                          </div>
                        </div>
                        <div
                          class="text-sm px-2 py-1.5 rounded bg-muted/30 font-mono break-all"
                        >
                          {{ property.value }}
                        </div>
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
