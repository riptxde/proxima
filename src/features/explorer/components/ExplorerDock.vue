<script setup lang="ts">
import { ref, watch, computed, onMounted, onUnmounted } from "vue";
import { Dock, DockIcon } from "@/components/ui/dock";
import { Play, Square, Search, Box, Route, Scroll } from "lucide-vue-next";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import { TooltipProvider } from "@/components/ui/tooltip";
import ClientsDialog from "./ClientsDialog.vue";
import SearchQueryDialog from "./SearchQueryDialog.vue";
import SearchResultsDialog from "./SearchResultsDialog.vue";
import LiquidGlass from "@/components/shared/LiquidGlass.vue";
import { useExplorer } from "../composables/useExplorer";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";
import { useNavigation } from "@/composables/useNavigation";
import { useLogger } from "@/composables/useLogger";
import { toast } from "vue-sonner";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

const { addLog } = useLogger();

const {
    selectedClient,
    selectedProperty,
    selectedItemPathString,
    selectedItemName,
    selectedItemId,
    selectedItemClassName,
    expStop,
    expDecompile,
} = useExplorer();

const { openFileAsTab } = useEditorTabs();
const { navigate } = useNavigation();

const clientsDialogOpen = ref(false);
const searchQueryDialogOpen = ref(false);
const searchResultsDialogOpen = ref(false);
const dockTooltipKey = ref(0);

const isClientSelected = computed(() => selectedClient.value !== null);
const isInstanceSelected = computed(
    () =>
        selectedItemName.value !== null &&
        selectedItemPathString.value !== null,
);
const isPropertySelected = computed(
    () =>
        selectedProperty.value !== null &&
        selectedItemPathString.value !== null,
);
const isScriptSelected = computed(() => {
    if (!selectedItemClassName.value || !selectedItemId.value) return false;
    return (
        selectedItemClassName.value === "LocalScript" ||
        selectedItemClassName.value === "ModuleScript"
    );
});

const handleStartExplorer = () => {
    if (isClientSelected.value) {
        toast.error("Could not start explorer", {
            description: "Explorer is already running",
        });
        return;
    }
    clientsDialogOpen.value = true;
};

const handleStopExplorer = async () => {
    if (!isClientSelected.value) {
        toast.error("Could not stop explorer", {
            description: "No explorer running",
        });
        return;
    }
    const username = selectedClient.value?.username;
    try {
        await expStop();
        toast.success("Explorer stopped", {
            description: `User: ${username}`,
        });
    } catch (error) {
        addLog("error", `Failed to stop explorer: ${error}`);
        toast.error("Failed to stop explorer", {
            description: String(error),
        });
    }
};

const handleSearchClick = () => {
    if (!isClientSelected.value) {
        toast.error("Could not search", {
            description: "No client started",
        });
        return;
    }
    searchQueryDialogOpen.value = true;
};

const handleSendInstanceNameToEditorClick = () => {
    if (!isInstanceSelected.value) {
        toast.error("Could not send instance path", {
            description: "No instance selected",
        });
        return;
    }

    const pathString = selectedItemPathString.value!;
    const code = `local instance = ${pathString}`;

    try {
        openFileAsTab("Instance Path", code);
        navigate("editor");
        toast.success("Instance path sent to editor", {
            description: `Instance: ${selectedItemName.value!}`,
        });
    } catch (error) {
        addLog("error", `Failed to send instance path to editor: ${error}`);
        toast.error("Failed to send instance path to editor", {
            description: String(error),
        });
    }
};

const handleSendCodeToEditorClick = () => {
    if (!isPropertySelected.value) {
        toast.error("Could not send code", {
            description: "No property selected",
        });
        return;
    }

    const property = selectedProperty.value!;

    try {
        openFileAsTab(`${property.name} Example`, property.propertyCode);
        navigate("editor");
        toast.success("Code sent to editor", {
            description: `Generated ${property.name} example code`,
        });
    } catch (error) {
        addLog("error", `Failed to send code to editor: ${error}`);
        toast.error("Failed to send code to editor", {
            description: String(error),
        });
    }
};

const handleDecompileClick = async () => {
    if (!isScriptSelected.value) {
        toast.error("Could not decompile", {
            description: "Select a LocalScript or ModuleScript",
        });
        return;
    }

    try {
        await expDecompile(selectedItemId.value!);
    } catch (error) {
        addLog("error", `Failed to decompile script: ${error}`);
        toast.error("Failed to decompile script", {
            description: String(error),
        });
    }
};

const handleSearchResultsReady = () => {
    searchResultsDialogOpen.value = true;
};

// Listen for decompiled script event
let unlistenDecompiledScript: UnlistenFn | null = null;

onMounted(async () => {
    unlistenDecompiledScript = await listen<{
        id: number;
        source: string;
    }>("explorer-decompiled-script", (event) => {
        const scriptName = selectedItemName.value || "Decompiled Script";
        try {
            openFileAsTab(scriptName, event.payload.source);
            navigate("editor");
            toast.success("Script decompiled and sent to editor", {
                description: `Script: ${scriptName}`,
            });
        } catch (error) {
            addLog(
                "error",
                `Failed to send decompiled script to editor: ${error}`,
            );
            toast.error("Failed to send decompiled script to editor", {
                description: String(error),
            });
        }
    });
});

onUnmounted(() => {
    unlistenDecompiledScript?.();
});

// Remount dock tooltips when dialog closes
// This is absolutely necessary otherwise, tooltips stop working after a dialog opens
watch(
    [clientsDialogOpen, searchQueryDialogOpen, searchResultsDialogOpen],
    (newValues, oldValues) => {
        const wasOpen = oldValues.some((val) => val);
        const isOpen = newValues.some((val) => val);
        if (wasOpen && !isOpen) {
            dockTooltipKey.value++;
        }
    },
);
</script>

<template>
    <div class="flex items-center justify-center p-4">
        <LiquidGlass>
            <TooltipProvider :key="dockTooltipKey">
                <Dock class="m-0!">
                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon
                                @click="handleStartExplorer"
                                :class="{
                                    'opacity-30 cursor-not-allowed':
                                        isClientSelected,
                                }"
                            >
                                <Play
                                    class="size-5 text-app-shell-foreground transition-opacity"
                                    :class="{
                                        'opacity-60 group-hover:opacity-100':
                                            !isClientSelected,
                                        'opacity-30': isClientSelected,
                                    }"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Start Explorer</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon
                                @click="handleStopExplorer"
                                :class="{
                                    'opacity-30 cursor-not-allowed':
                                        !isClientSelected,
                                }"
                            >
                                <Square
                                    class="size-5 text-app-shell-foreground transition-opacity"
                                    :class="{
                                        'opacity-60 group-hover:opacity-100':
                                            isClientSelected,
                                        'opacity-30': !isClientSelected,
                                    }"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Stop Explorer</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon
                                @click="handleSearchClick"
                                :class="{
                                    'opacity-30 cursor-not-allowed':
                                        !isClientSelected,
                                }"
                            >
                                <Search
                                    class="size-5 text-app-shell-foreground transition-opacity"
                                    :class="{
                                        'opacity-60 group-hover:opacity-100':
                                            isClientSelected,
                                        'opacity-30': !isClientSelected,
                                    }"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Search for Instances</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon
                                @click="handleSendInstanceNameToEditorClick"
                                :class="{
                                    'opacity-30 cursor-not-allowed':
                                        !isInstanceSelected,
                                }"
                            >
                                <Route
                                    class="size-5 text-app-shell-foreground transition-opacity"
                                    :class="{
                                        'opacity-60 group-hover:opacity-100':
                                            isInstanceSelected,
                                        'opacity-30': !isInstanceSelected,
                                    }"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Generate Instance Path</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon
                                @click="handleSendCodeToEditorClick"
                                :class="{
                                    'opacity-30 cursor-not-allowed':
                                        !isPropertySelected,
                                }"
                            >
                                <Box
                                    class="size-5 text-app-shell-foreground transition-opacity"
                                    :class="{
                                        'opacity-60 group-hover:opacity-100':
                                            isPropertySelected,
                                        'opacity-30': !isPropertySelected,
                                    }"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Generate Property Code</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon
                                @click="handleDecompileClick"
                                :class="{
                                    'opacity-30 cursor-not-allowed':
                                        !isScriptSelected,
                                }"
                            >
                                <Scroll
                                    class="size-5 text-app-shell-foreground transition-opacity"
                                    :class="{
                                        'opacity-60 group-hover:opacity-100':
                                            isScriptSelected,
                                        'opacity-30': !isScriptSelected,
                                    }"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Decompile Selected Script</p>
                        </TooltipContent>
                    </Tooltip>
                </Dock>
            </TooltipProvider>
        </LiquidGlass>

        <ClientsDialog v-model:open="clientsDialogOpen" />
        <SearchQueryDialog
            v-model:open="searchQueryDialogOpen"
            @results-ready="handleSearchResultsReady"
        />
        <SearchResultsDialog v-model:open="searchResultsDialogOpen" />
    </div>
</template>
