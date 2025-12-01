<script setup lang="ts">
import { ref } from "vue";
import LiquidGlass from "@/components/shared/LiquidGlass.vue";
import SaveDialog from "@/components/shared/SaveDialog.vue";
import ClientsDialog from "../ClientsDialog/ClientsDialog.vue";
import DockActions from "./DockActions.vue";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";
import { useFileOperations } from "@/features/editor/composables/useFileOperations";
import { useClients } from "@/features/editor/composables/useClients";
import { useExecutor } from "@/features/editor/composables/useExecutor";

const { clearActiveTab, getActiveTabContent } = useEditorTabs();
const { getEnabledClientIds } = useClients();
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

const handleExecute = async () => {
    const script = getActiveTabContent();
    const clientIds = getEnabledClientIds();
    await executeScript(script, clientIds);
};

const handleClientsClick = () => {
    clientsDialogOpen.value = true;
};
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
            <DockActions
                @execute="handleExecute"
                @clear="clearActiveTab"
                @open="handleOpenScript"
                @save="handleSaveClick"
                @clients="handleClientsClick"
            />
        </LiquidGlass>

        <!-- Save Dialog -->
        <SaveDialog v-model:open="saveDialogOpen" @save="handleSave" />

        <!-- Clients Dialog -->
        <ClientsDialog v-model:open="clientsDialogOpen" />
    </div>
</template>
