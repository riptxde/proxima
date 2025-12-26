<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Input } from "@/components/ui/input";
import {
    Search,
    ListTree,
    ScanSearch,
    BookOpen,
    FileText,
} from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { openUrl } from "@tauri-apps/plugin-opener";
import { toast } from "vue-sonner";
import ExplorerItem from "./ExplorerItem.vue";
import ExplorerDock from "./ExplorerDock.vue";
import { useExplorer } from "../composables/useExplorer";
import { useLogger } from "@/composables/useLogger";

const { addLog } = useLogger();

const {
    selectedClient,
    explorerItems,
    availableClients,
    selectedItemId,
    selectedItemClassName,
    selectedItemProperties,
    selectedProperty,
    expandedIds,
    initializeListeners,
    cleanupListeners,
    expGetTree,
    selectProperty,
} = useExplorer();

// Property search filter
const propertySearchQuery = ref("");

// Separate normal and special properties with Name and ClassName prioritized
const normalProperties = computed(() => {
    const filtered = selectedItemProperties.value.filter(
        (p) => !p.hidden && !p.notScriptable,
    );

    // Apply search filter
    const searched = propertySearchQuery.value.trim()
        ? filtered.filter((p) =>
              p.name
                  .toLowerCase()
                  .includes(propertySearchQuery.value.toLowerCase()),
          )
        : filtered;

    // Sort with Name first, ClassName second, then alphabetically
    return searched.sort((a, b) => {
        if (a.name === "Name") return -1;
        if (b.name === "Name") return 1;
        if (a.name === "ClassName") return -1;
        if (b.name === "ClassName") return 1;
        return a.name.localeCompare(b.name);
    });
});

const specialProperties = computed(() => {
    const filtered = selectedItemProperties.value.filter(
        (p) => p.hidden || p.notScriptable,
    );

    // Apply search filter
    const searched = propertySearchQuery.value.trim()
        ? filtered.filter((p) =>
              p.name
                  .toLowerCase()
                  .includes(propertySearchQuery.value.toLowerCase()),
          )
        : filtered;

    return searched.sort((a, b) => a.name.localeCompare(b.name));
});

onMounted(async () => {
    await initializeListeners();

    // Fetch initial client list
    try {
        const clients = await invoke("get_attached_clients");
        availableClients.value = clients as any[];
    } catch (error) {
        addLog("error", `Failed to get clients: ${error}`);
    }

    // Request initial tree if explorer is active
    if (selectedClient.value) {
        // Use existing expanded IDs to preserve expansion state on remount
        expGetTree(Array.from(expandedIds.value));
    }
});

onUnmounted(() => {
    cleanupListeners();
});

const openOfficialDocs = async () => {
    if (!selectedItemClassName.value) return;

    const url = `https://create.roblox.com/docs/reference/engine/classes/${selectedItemClassName.value}`;
    try {
        await openUrl(url);
        addLog("info", `Opening official docs: ${selectedItemClassName.value}`);
    } catch (error) {
        toast.error("Failed to open documentation", {
            description: String(error),
        });
        addLog("error", `Failed to open official docs: ${error}`);
    }
};

const openUnofficialDocs = async () => {
    if (!selectedItemClassName.value) return;

    const url = `https://robloxapi.github.io/ref/class/${selectedItemClassName.value}`;
    try {
        await openUrl(url);
        addLog(
            "info",
            `Opening unofficial docs: ${selectedItemClassName.value}`,
        );
    } catch (error) {
        toast.error("Failed to open documentation", {
            description: String(error),
        });
        addLog("error", `Failed to open unofficial docs: ${error}`);
    }
};
</script>

<template>
    <div
        data-page="explorer"
        class="h-full overflow-hidden flex flex-col rounded-lg border border-border shadow-sm"
    >
        <div class="flex-1 overflow-hidden p-4 pb-0 flex flex-col gap-4">
            <!-- Resizable Panels -->
            <ResizablePanelGroup direction="horizontal" class="flex-1 gap-4">
                <!-- Explorer Tree Panel -->
                <ResizablePanel :default-size="45" :min-size="25">
                    <div
                        class="h-full flex flex-col bg-card rounded-lg border border-border"
                    >
                        <div class="flex-1 overflow-y-auto p-2">
                            <!-- Empty State -->
                            <div
                                v-if="
                                    !selectedClient ||
                                    explorerItems.length === 0
                                "
                                class="h-full flex items-center justify-center text-center p-8"
                            >
                                <div class="text-muted-foreground">
                                    <ScanSearch
                                        class="w-12 h-12 mx-auto mb-3 opacity-30"
                                    />
                                    <p class="text-sm">No explorer data</p>
                                    <p class="text-xs mt-1">
                                        Start the explorer to view instances
                                    </p>
                                </div>
                            </div>

                            <!-- Explorer Items -->
                            <ExplorerItem
                                v-else
                                v-for="item in explorerItems"
                                :key="item.id"
                                :item="item"
                            />
                        </div>
                    </div>
                </ResizablePanel>

                <ResizableHandle with-handle />

                <!-- Properties Panel -->
                <ResizablePanel :default-size="55" :min-size="25">
                    <div
                        class="h-full flex flex-col bg-card rounded-lg border border-border"
                    >
                        <!-- Property Search Bar -->
                        <div
                            v-if="selectedItemId"
                            class="px-4 pt-4 pb-4 border-b border-border"
                        >
                            <div class="relative">
                                <Search
                                    class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground pointer-events-none"
                                />
                                <Input
                                    v-model="propertySearchQuery"
                                    placeholder="Search"
                                    class="pl-9 h-10 bg-muted/50 border-border focus-visible:ring-sidebar-primary"
                                />
                            </div>
                        </div>

                        <div class="flex-1 overflow-y-auto">
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
                                            class="p-2 rounded-lg border cursor-pointer transition-all"
                                            :class="
                                                selectedProperty?.name ===
                                                property.name
                                                    ? 'bg-blue-500/30 border-blue-500/50 hover:bg-blue-500/40'
                                                    : 'border-border hover:bg-accent/10'
                                            "
                                            @click="selectProperty(property)"
                                        >
                                            <div
                                                class="flex items-start justify-between gap-2 mb-2"
                                            >
                                                <div
                                                    class="flex items-center gap-2 flex-wrap"
                                                >
                                                    <svg
                                                        width="20"
                                                        height="20"
                                                        viewBox="0 0 20 20"
                                                        fill="none"
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        class="shrink-0 pb-0.5"
                                                    >
                                                        <path
                                                            fill-rule="evenodd"
                                                            clip-rule="evenodd"
                                                            d="M4.89453 9.38443V15.4702C4.89453 16.0225 5.34225 16.4702 5.89453 16.4702H11.9803C12.3781 16.4702 12.7597 16.3122 13.041 16.0309L15.4552 13.6167C15.7365 13.3354 15.8945 12.9538 15.8945 12.556V6.97021C15.8945 6.14179 15.223 5.47021 14.3945 5.47021H8.80874C8.41092 5.47021 8.02939 5.62825 7.74808 5.90955L5.33387 8.32377C5.05257 8.60507 4.89453 8.9866 4.89453 9.38443ZM8.80874 6.47021C8.67614 6.47021 8.54896 6.52289 8.45519 6.61666L6.60164 8.47021H11.8945C11.9841 8.47021 12.0709 8.48199 12.1536 8.50409L14.1874 6.47021H8.80874ZM14.8945 7.17732L12.8607 9.21119C12.8828 9.29381 12.8945 9.38064 12.8945 9.47021V14.7631L14.7481 12.9096C14.8419 12.8158 14.8945 12.6886 14.8945 12.556V7.17732Z"
                                                            fill="#1AC8FF"
                                                        ></path>
                                                    </svg>
                                                    <span
                                                        class="text-sm font-medium"
                                                        >{{
                                                            property.name
                                                        }}</span
                                                    >
                                                    <span
                                                        class="text-xs px-1.5 py-0.5 rounded bg-muted/50 text-muted-foreground font-mono"
                                                        >{{
                                                            property.type ===
                                                            "className"
                                                                ? "ClassName"
                                                                : property.type
                                                        }}</span
                                                    >
                                                </div>
                                                <div
                                                    class="flex items-center gap-1.5 shrink-0"
                                                >
                                                    <span
                                                        v-if="
                                                            property.deprecated
                                                        "
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
                                                class="text-xs px-2 py-1.5 rounded bg-muted/30 font-mono break-all"
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
                                            class="p-2 rounded-lg border cursor-pointer transition-all"
                                            :class="
                                                selectedProperty?.name ===
                                                property.name
                                                    ? 'bg-blue-500/30 border-blue-500/50 hover:bg-blue-500/40'
                                                    : 'border-border hover:bg-accent/10'
                                            "
                                            @click="selectProperty(property)"
                                        >
                                            <div
                                                class="flex items-start justify-between gap-2 mb-2"
                                            >
                                                <div
                                                    class="flex items-center gap-2 flex-wrap"
                                                >
                                                    <svg
                                                        width="20"
                                                        height="20"
                                                        viewBox="0 0 20 20"
                                                        fill="none"
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        class="shrink-0 pb-0.5"
                                                    >
                                                        <path
                                                            fill-rule="evenodd"
                                                            clip-rule="evenodd"
                                                            d="M4.89453 9.38443V15.4702C4.89453 16.0225 5.34225 16.4702 5.89453 16.4702H11.9803C12.3781 16.4702 12.7597 16.3122 13.041 16.0309L15.4552 13.6167C15.7365 13.3354 15.8945 12.9538 15.8945 12.556V6.97021C15.8945 6.14179 15.223 5.47021 14.3945 5.47021H8.80874C8.41092 5.47021 8.02939 5.62825 7.74808 5.90955L5.33387 8.32377C5.05257 8.60507 4.89453 8.9866 4.89453 9.38443ZM8.80874 6.47021C8.67614 6.47021 8.54896 6.52289 8.45519 6.61666L6.60164 8.47021H11.8945C11.9841 8.47021 12.0709 8.48199 12.1536 8.50409L14.1874 6.47021H8.80874ZM14.8945 7.17732L12.8607 9.21119C12.8828 9.29381 12.8945 9.38064 12.8945 9.47021V14.7631L14.7481 12.9096C14.8419 12.8158 14.8945 12.6886 14.8945 12.556V7.17732Z"
                                                            fill="#fb923c"
                                                        ></path>
                                                    </svg>
                                                    <span
                                                        class="text-sm font-medium"
                                                        >{{
                                                            property.name
                                                        }}</span
                                                    >
                                                    <span
                                                        class="text-xs px-1.5 py-0.5 rounded bg-muted/50 text-muted-foreground font-mono"
                                                        >{{
                                                            property.type ===
                                                            "className"
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
                                                        v-if="
                                                            property.notScriptable
                                                        "
                                                        class="text-xs px-1.5 py-0.5 rounded bg-blue-500/20 text-blue-400 font-medium"
                                                        >Unscriptable</span
                                                    >
                                                    <span
                                                        v-if="
                                                            property.deprecated
                                                        "
                                                        class="text-xs px-1.5 py-0.5 rounded bg-orange-500/20 text-orange-400 font-medium"
                                                        >Deprecated</span
                                                    >
                                                </div>
                                            </div>
                                            <div
                                                class="text-xs px-2 py-1.5 rounded bg-muted/30 font-mono break-all"
                                            >
                                                {{ property.value }}
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <!-- Documentation Section -->
                                <div v-if="selectedItemClassName">
                                    <h3
                                        class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3"
                                    >
                                        Documentation
                                    </h3>
                                    <div class="space-y-3">
                                        <div class="flex gap-2">
                                            <Button
                                                variant="outline"
                                                size="sm"
                                                class="flex-1 gap-2"
                                                @click="openOfficialDocs"
                                            >
                                                <BookOpen class="w-4 h-4" />
                                                Official Docs
                                            </Button>
                                            <Button
                                                variant="outline"
                                                size="sm"
                                                class="flex-1 gap-2"
                                                @click="openUnofficialDocs"
                                            >
                                                <FileText class="w-4 h-4" />
                                                Unofficial Docs
                                            </Button>
                                        </div>
                                        <p
                                            class="text-xs text-muted-foreground/80 italic"
                                        >
                                            Note: The official documentation
                                            provides better examples, while the
                                            unofficial docs contain
                                            documentation on properties and
                                            events hidden from the official
                                            documentation.
                                        </p>
                                    </div>
                                </div>
                            </div>
                            <div
                                v-else
                                class="h-full flex items-center justify-center text-center p-8"
                            >
                                <div class="text-muted-foreground">
                                    <ListTree
                                        class="w-12 h-12 mx-auto mb-3 opacity-30"
                                    />
                                    <p class="text-sm">No instance selected</p>
                                    <p class="text-xs mt-1">
                                        Select an instance to view its
                                        properties
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </ResizablePanel>
            </ResizablePanelGroup>
        </div>

        <ExplorerDock />
    </div>
</template>
