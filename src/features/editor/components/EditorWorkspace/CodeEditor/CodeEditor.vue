<script setup lang="ts">
import { computed } from "vue";
import { Card } from "@/components/ui/card";
import { MonacoEditor } from "@/components/shared/MonacoEditor";
import TabBar from "./TabBar/TabBar.vue";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";

const {
    tabs,
    activeTabId,
    addTab,
    closeTab,
    selectTab,
    renameTab,
    updateTabContent,
} = useEditorTabs();

const activeTab = computed(() =>
    tabs.value.find((tab) => tab.id === activeTabId.value),
);

const scriptContent = computed({
    get: () => activeTab.value?.content || "",
    set: (value: string) => {
        const tab = activeTab.value;
        if (tab) {
            updateTabContent(tab.id, value);
        }
    },
});
</script>

<template>
    <Card class="h-full overflow-hidden p-2 flex flex-col gap-3">
        <!-- Tab Bar -->
        <TabBar
            :tabs="tabs"
            :active-tab-id="activeTabId"
            @add-tab="addTab"
            @select-tab="selectTab"
            @rename-tab="renameTab"
            @close-tab="closeTab"
        />

        <!-- Editor -->
        <div class="flex-1 overflow-hidden">
            <MonacoEditor v-model="scriptContent" />
        </div>
    </Card>
</template>
