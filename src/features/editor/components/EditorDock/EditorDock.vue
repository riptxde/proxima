<script setup lang="ts">
import { ref, watch } from "vue";
import LiquidGlass from "@/components/shared/LiquidGlass.vue";
import SaveDialog from "@/components/shared/SaveDialog.vue";
import ClientsDialog from "../ClientsDialog/ClientsDialog.vue";
import DockActions from "./DockActions.vue";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";
import { useFileOperations } from "@/features/editor/composables/useFileOperations";
import { useClients } from "@/features/editor/composables/useClients";
import { useExecutor } from "@/features/editor/composables/useExecutor";

const props = defineProps<{
    triggerExecute: number;
    triggerOpen: number;
    triggerSave: number;
    triggerClients: number;
    triggerClear: number;
}>();

const { clearActiveTab, getActiveTabContent } = useEditorTabs();
const { getSelectedClientIds, selectedCount } = useClients();
const { executeScript } = useExecutor();

const {
    fileInputRef,
    saveDialogOpen,
    handleOpenScript,
    handleFileChange,
    handleSaveClick,
    handleSave,
} = useFileOperations();

const clientsDialogOpen = ref(false);
const dockTooltipKey = ref(0);

const handleExecute = async () => {
    const script = getActiveTabContent();
    const clientIds = getSelectedClientIds();
    await executeScript(script, clientIds);
};

const handleClientsClick = () => {
    clientsDialogOpen.value = true;
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
        clearActiveTab();
    },
);

// Remount dock tooltips when any dialog closes
// This is absolutely necessary otherwise, tooltips stop working after a dialog opens
watch(
    [saveDialogOpen, clientsDialogOpen],
    ([newSave, newClients], [oldSave, oldClients]) => {
        if ((oldSave && !newSave) || (oldClients && !newClients)) {
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
                    @clear="clearActiveTab"
                    @open="handleOpenScript"
                    @save="handleSaveClick"
                    @clients="handleClientsClick"
                />
            </TooltipProvider>
        </LiquidGlass>

        <!-- Save Dialog -->
        <SaveDialog v-model:open="saveDialogOpen" @save="handleSave" />

        <!-- Clients Dialog -->
        <ClientsDialog v-model:open="clientsDialogOpen" />
    </div>
</template>
