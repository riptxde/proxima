<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { Card } from "@/components/ui/card";
import { MonacoEditor } from "@/components/shared/MonacoEditor";
import type * as Monaco from "monaco-editor";
import TabBar from "./TabBar/TabBar.vue";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";
import { useSettings } from "@/features/settings/composables/useSettings";

const {
    tabs,
    activeTabId,
    addTab,
    closeTab,
    selectTab,
    renameTab,
    updateTabContent,
} = useEditorTabs();

const { editorSettings } = useSettings();

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

// Monaco editor instance
const editorInstance = ref<Monaco.editor.IStandaloneCodeEditor | null>(null);

const handleEditorMount = (editor: Monaco.editor.IStandaloneCodeEditor) => {
    editorInstance.value = editor;
};

// Build Monaco options from settings
const editorOptions = computed(() => ({
    automaticLayout: true,
    formatOnType: false,
    formatOnPaste: false,
    minimap: {
        enabled: editorSettings.value.minimap,
    },
    scrollbar: {
        verticalScrollbarSize: 12,
        horizontalScrollbarSize: 12,
    },
    fontSize: editorSettings.value.fontSize,
    fontFamily: `${editorSettings.value.font}, ui-monospace, monospace`,
    fontLigatures: editorSettings.value.fontLigatures,
    wordWrap: editorSettings.value.wordWrap
        ? ("on" as const)
        : ("off" as const),
    lineNumbers: "on" as const,
    roundedSelection: true,
    padding: {
        top: 0,
        bottom: 12,
    },
    overviewRulerLanes: 0,
    hideCursorInOverviewRuler: true,
    scrollBeyondLastLine: false,
}));

// Watch settings changes and update Monaco manually
watch(
    () => editorSettings.value,
    (settings) => {
        if (!editorInstance.value) return;

        editorInstance.value.updateOptions({
            minimap: { enabled: settings.minimap },
            fontSize: settings.fontSize,
            fontFamily: `${settings.font}, ui-monospace, monospace`,
            fontLigatures: settings.fontLigatures,
            wordWrap: settings.wordWrap ? "on" : "off",
        });
    },
    { deep: true },
);
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
            <MonacoEditor
                v-model="scriptContent"
                :options="editorOptions"
                @mount="handleEditorMount"
            />
        </div>
    </Card>
</template>
