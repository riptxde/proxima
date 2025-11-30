<script setup lang="ts">
import { TooltipProvider } from "@/components/ui/tooltip";
import Tab from "./Tab.vue";
import TabAddButton from "./TabAddButton.vue";
import type { Tab as TabType } from "@/features/editor/types/tab";

interface Props {
    tabs: TabType[];
    activeTabId: number;
}

defineProps<Props>();

const emit = defineEmits<{
    addTab: [];
    selectTab: [tabId: number];
    renameTab: [tabId: number, newName: string];
    closeTab: [tabId: number];
}>();
</script>

<template>
    <div
        class="flex items-center gap-1 bg-tab-bar rounded-md p-1.5 overflow-x-auto"
    >
        <TooltipProvider>
            <Tab
                v-for="tab in tabs"
                :key="tab.id"
                :id="tab.id"
                :name="tab.name"
                :is-active="tab.id === activeTabId"
                :show-close="tabs.length > 1"
                :file-path="tab.filePath"
                @select="(tabId) => emit('selectTab', tabId)"
                @rename="(tabId, newName) => emit('renameTab', tabId, newName)"
                @close="(tabId) => emit('closeTab', tabId)"
            />
        </TooltipProvider>

        <TabAddButton @click="emit('addTab')" />
    </div>
</template>
