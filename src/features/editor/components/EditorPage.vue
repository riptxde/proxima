<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import StarsBackground from "@/components/ui/bg-stars/StarsBackground.vue";
import EditorWorkspace from "./EditorWorkspace/EditorWorkspace.vue";
import EditorDock from "./EditorDock/EditorDock.vue";

// Refs to trigger actions in child components
const triggerExecute = ref(0);
const triggerOpen = ref(0);
const triggerSave = ref(0);
const triggerClients = ref(0);
const triggerClear = ref(0);
const triggerLogs = ref(0);
const triggerNewTab = ref(0);
const triggerCloseTab = ref(0);

const handleKeyDown = async (event: KeyboardEvent) => {
    // Handle Alt key shortcuts
    if (event.altKey) {
        switch (event.key.toLowerCase()) {
            case "r":
                event.preventDefault();
                window.location.reload();
                return;
            case "c":
                event.preventDefault();
                triggerClients.value++;
                return;
            case "x":
                event.preventDefault();
                triggerClear.value++;
                return;
        }
    }

    // Check if Ctrl key is pressed
    if (!event.ctrlKey) return;

    switch (event.key.toLowerCase()) {
        case "r":
            // Prevent browser refresh
            event.preventDefault();
            // Trigger execute action
            triggerExecute.value++;
            break;

        case "o":
            // Prevent default browser open file dialog
            event.preventDefault();
            // Trigger open action
            triggerOpen.value++;
            break;

        case "s":
            // Prevent default browser save dialog
            event.preventDefault();
            // Trigger save action
            triggerSave.value++;
            break;

        case "b":
            // Prevent default browser behavior
            event.preventDefault();
            // Trigger logs toggle
            triggerLogs.value++;
            break;

        case "t":
            // Prevent default browser new tab
            event.preventDefault();
            // Trigger new tab action
            triggerNewTab.value++;
            break;

        case "w":
            // Prevent default browser close tab
            event.preventDefault();
            // Trigger close tab action
            triggerCloseTab.value++;
            break;
    }
};

onMounted(() => {
    window.addEventListener("keydown", handleKeyDown);
});

onUnmounted(() => {
    window.removeEventListener("keydown", handleKeyDown);
});
</script>

<template>
    <div
        class="h-full overflow-hidden flex flex-col relative bg-card rounded-lg border border-border shadow-sm"
    >
        <!-- Stars Background -->
        <div class="absolute inset-0 z-0 pointer-events-none rounded-lg">
            <StarsBackground :factor="0.05" :speed="50" star-color="#fff" />
        </div>

        <!-- Editor Workspace -->
        <div class="relative z-10 flex-1 flex flex-col overflow-hidden">
            <EditorWorkspace
                :trigger-new-tab="triggerNewTab"
                :trigger-close-tab="triggerCloseTab"
            />
            <EditorDock
                :trigger-execute="triggerExecute"
                :trigger-open="triggerOpen"
                :trigger-save="triggerSave"
                :trigger-clients="triggerClients"
                :trigger-clear="triggerClear"
                :trigger-logs="triggerLogs"
            />
        </div>
    </div>
</template>
