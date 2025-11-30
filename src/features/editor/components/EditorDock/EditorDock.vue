<script setup lang="ts">
import LiquidGlass from "@/components/shared/LiquidGlass.vue";
import SaveDialog from "@/components/shared/SaveDialog.vue";
import DockActions from "./DockActions.vue";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";
import { useFileOperations } from "@/features/editor/composables/useFileOperations";

const { clearActiveTab } = useEditorTabs();

const {
    fileInputRef,
    saveDialogOpen,
    handleOpenScript,
    handleFileChange,
    handleSaveClick,
    handleSave,
} = useFileOperations();

const handleExecute = () => {
    // TODO: Implement script execution
    console.log("Execute script");
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
            />
        </LiquidGlass>

        <!-- Save Dialog -->
        <SaveDialog v-model:open="saveDialogOpen" @save="handleSave" />
    </div>
</template>
