<script setup lang="ts">
import { ref, watch } from "vue";
import LiquidGlass from "@/components/shared/LiquidGlass.vue";
import SaveDialog from "@/components/shared/SaveDialog.vue";
import ClientsDialog from "../ClientsDialog/ClientsDialog.vue";
import ClearConfirmDialog from "./ClearConfirmDialog.vue";
import DockActions from "./DockActions.vue";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";
import { useFileOperations } from "@/features/editor/composables/useFileOperations";
import { useExecutorClients } from "@/features/editor/composables/useExecutorClients";
import { useExecutor } from "@/features/editor/composables/useExecutor";
import { useEditorLogs } from "@/features/editor/composables/useEditorLogs";

const props = defineProps<{
    triggerExecute: number;
    triggerOpen: number;
    triggerSave: number;
    triggerClients: number;
    triggerClear: number;
    triggerLogs: number;
}>();

const { clearActiveTab, getActiveTabContent } = useEditorTabs();
const { getSelectedClientIds, selectedCount } = useExecutorClients();
const { executeScript } = useExecutor();
const { toggleLogs } = useEditorLogs();

const {
    saveDialogOpen,
    handleOpenScript,
    handleFileChange,
    handleSaveClick,
    handleSave,
} = useFileOperations();

const clientsDialogOpen = ref(false);
const clearConfirmDialogOpen = ref(false);
const dockTooltipKey = ref(0);

const handleExecute = async () => {
    const script = getActiveTabContent();
    const clientIds = getSelectedClientIds();
    await executeScript(script, clientIds);
};

const handleClientsClick = () => {
    clientsDialogOpen.value = true;
};

const handleClearClick = () => {
    clearConfirmDialogOpen.value = true;
};

const handleClearConfirm = () => {
    clearActiveTab();
    clearConfirmDialogOpen.value = false;
};

// Watch for keyboard shortcut triggers from parent
watch(
    () => props.triggerExecute,
    () => {
        handleExecute();
    },
);

watch(
    () => props.triggerOpen,
    () => {
        handleOpenScript();
    },
);

watch(
    () => props.triggerSave,
    () => {
        handleSaveClick();
    },
);

watch(
    () => props.triggerClients,
    () => {
        handleClientsClick();
    },
);

watch(
    () => props.triggerClear,
    () => {
        handleClearClick();
    },
);

watch(
    () => props.triggerLogs,
    () => {
        toggleLogs();
    },
);

// Remount dock tooltips when any dialog closes
// This is absolutely necessary otherwise, tooltips stop working after a dialog opens
watch(
    [saveDialogOpen, clientsDialogOpen, clearConfirmDialogOpen],
    ([newSave, newClients, newClear], [oldSave, oldClients, oldClear]) => {
        if (
            (oldSave && !newSave) ||
            (oldClients && !newClients) ||
            (oldClear && !newClear)
        ) {
            dockTooltipKey.value++;
        }
    },
);
</script>

<template>
    <div class="flex justify-center items-center py-4">
        <!-- Hidden file input -->
        <input
            ref="fileInputRef"
            type="file"
            accept=".lua,.luau,.txt"
            @change="handleFileChange"
            class="hidden"
        />

        <LiquidGlass>
            <TooltipProvider :key="dockTooltipKey">
                <DockActions
                    :selected-count="selectedCount"
                    @execute="handleExecute"
                    @clear="handleClearClick"
                    @open="handleOpenScript"
                    @save="handleSaveClick"
                    @clients="handleClientsClick"
                    @toggle-logs="toggleLogs"
                />
            </TooltipProvider>
        </LiquidGlass>

        <!-- Save Dialog -->
        <SaveDialog v-model:open="saveDialogOpen" @save="handleSave" />

        <!-- Clients Dialog -->
        <ClientsDialog v-model:open="clientsDialogOpen" />

        <!-- Clear Confirm Dialog -->
        <ClearConfirmDialog
            v-model:open="clearConfirmDialogOpen"
            @confirm="handleClearConfirm"
        />
    </div>
</template>
