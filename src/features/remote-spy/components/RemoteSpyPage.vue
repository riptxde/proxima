<script setup lang="ts">
import { onMounted, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";
import { FunctionSquare } from "lucide-vue-next";
import RemoteFilters from "./RemoteFilters.vue";
import RemotesList from "./RemotesList.vue";
import RemoteCallsList from "./RemoteCallsList.vue";
import CallDetails from "./CallDetails.vue";
import RemoteSpyDock from "./RemoteSpyDock.vue";
import { useRemoteSpy } from "../composables/useRemoteSpy";
import { useLogger } from "@/composables/useLogger";

const { addLog } = useLogger();

const {
    remotes,
    selectedRemote,
    selectedCall,
    selectedCallId,
    filters,
    availableClients,
    selectRemote,
    deselectRemote,
    selectCall,
    toggleDirectionFilter,
    toggleTypeFilter,
    setSearchFilter,
    initializeListeners,
    cleanupListeners,
} = useRemoteSpy();

onMounted(async () => {
    await initializeListeners();

    // Fetch initial client list
    try {
        const clients = await invoke("get_attached_clients");
        availableClients.value = clients as any[];
    } catch (error) {
        addLog("error", `Failed to get clients: ${error}`);
    }
});

onBeforeUnmount(() => {
    cleanupListeners();
});
</script>

<template>
    <div
        data-page="remote-spy"
        class="h-full overflow-hidden flex flex-col rounded-lg border border-border shadow-sm"
    >
        <div class="flex-1 overflow-hidden p-4 pb-0 flex flex-col gap-4">
            <!-- Resizable Panels -->
            <ResizablePanelGroup direction="horizontal" class="flex-1 gap-4">
                <!-- Remotes List Panel -->
                <ResizablePanel :default-size="45" :min-size="30">
                    <div
                        class="h-full flex flex-col bg-card rounded-lg border border-border"
                    >
                        <!-- Filter Header -->
                        <RemoteFilters
                            :active-directions="filters.directions"
                            :active-types="filters.types"
                            @toggle-direction="toggleDirectionFilter"
                            @toggle-type="toggleTypeFilter"
                        />

                        <!-- Content Area -->
                        <div class="flex-1 overflow-hidden flex flex-col">
                            <!-- Transition between remotes list and selected remote view -->
                            <Transition
                                mode="out-in"
                                enter-active-class="transition-all duration-200 ease-out"
                                leave-active-class="transition-all duration-200 ease-in"
                                enter-from-class="opacity-0 translate-y-4"
                                enter-to-class="opacity-100 translate-y-0"
                                leave-from-class="opacity-100 translate-y-0"
                                leave-to-class="opacity-0 -translate-y-4"
                            >
                                <!-- Remotes List View -->
                                <RemotesList
                                    v-if="!selectedRemote"
                                    key="remotes-list"
                                    :remotes="remotes"
                                    :search-query="filters.search"
                                    @select="selectRemote"
                                    @update:search-query="setSearchFilter"
                                />

                                <!-- Selected Remote View -->
                                <RemoteCallsList
                                    v-else
                                    key="selected-remote"
                                    :remote="selectedRemote"
                                    :selected-call-id="selectedCallId"
                                    @select-call="selectCall"
                                    @back="deselectRemote"
                                />
                            </Transition>
                        </div>
                    </div>
                </ResizablePanel>

                <ResizableHandle with-handle />

                <!-- Details Panel -->
                <ResizablePanel :default-size="55" :min-size="30">
                    <div
                        class="h-full flex flex-col bg-card rounded-lg border border-border"
                    >
                        <!-- Content -->
                        <div class="flex-1 overflow-y-auto">
                            <!-- Empty State -->
                            <div
                                v-if="!selectedCall"
                                class="h-full flex items-center justify-center text-center p-8"
                            >
                                <div class="text-muted-foreground">
                                    <FunctionSquare
                                        class="w-12 h-12 mx-auto mb-3 opacity-30"
                                    />
                                    <p class="text-sm">No call selected</p>
                                    <p class="text-xs mt-1">
                                        Select a call to view details
                                    </p>
                                </div>
                            </div>

                            <!-- Call Details -->
                            <CallDetails
                                v-else
                                :remote="selectedRemote!"
                                :call="selectedCall"
                            />
                        </div>
                    </div>
                </ResizablePanel>
            </ResizablePanelGroup>
        </div>

        <!-- Dock -->
        <RemoteSpyDock />
    </div>
</template>
