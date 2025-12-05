<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { Card } from "@/components/ui/card";
import {
    ResizablePanelGroup,
    ResizablePanel,
    ResizableHandle,
} from "@/components/ui/resizable";
import { MonacoEditor } from "@/components/shared/MonacoEditor";
import type * as Monaco from "monaco-editor";
import TabBar from "./TabBar/TabBar.vue";
import EditorLogsPane from "./EditorLogsPane.vue";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";
import { useSettings } from "@/features/settings/composables/useSettings";
import { useEditorLogs } from "@/features/editor/composables/useEditorLogs";
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
} from "@/components/ui/alert-dialog";

const props = defineProps<{
    triggerNewTab: number;
    triggerCloseTab: number;
}>();

const {
    tabs,
    activeTabId,
    addTab,
    closeTab,
    selectTab,
    renameTab,
    updateTabContent,
    hasUnsavedChanges,
} = useEditorTabs();

const closeConfirmDialogOpen = ref(false);
const tabToClose = ref<number | null>(null);

const handleCloseTabClick = (tabId: number) => {
    if (hasUnsavedChanges(tabId)) {
        tabToClose.value = tabId;
        closeConfirmDialogOpen.value = true;
    } else {
        closeTab(tabId);
    }
};

const handleCloseConfirm = () => {
    if (tabToClose.value !== null) {
        closeTab(tabToClose.value);
        tabToClose.value = null;
    }
    closeConfirmDialogOpen.value = false;
};

const { editorSettings } = useSettings();
const { showLogs } = useEditorLogs();

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

// Watch for new tab trigger
watch(
    () => props.triggerNewTab,
    () => {
        addTab();
    },
);

// Watch for close tab trigger
watch(
    () => props.triggerCloseTab,
    () => {
        if (activeTabId.value) {
            handleCloseTabClick(activeTabId.value);
        }
    },
);
</script>

<template>
    <ResizablePanelGroup direction="vertical" class="h-full">
        <!-- Editor Panel -->
        <ResizablePanel :default-size="65" :min-size="30">
            <Card
                class="h-full overflow-hidden p-2 flex flex-col gap-3"
                :class="{ 'rounded-b-none': showLogs }"
            >
                <!-- Tab Bar -->
                <TabBar
                    :tabs="tabs"
                    :active-tab-id="activeTabId"
                    @add-tab="addTab"
                    @select-tab="selectTab"
                    @rename-tab="renameTab"
                    @close-tab="handleCloseTabClick"
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
        </ResizablePanel>

        <template v-if="showLogs">
            <ResizableHandle with-handle />

            <!-- Logs Panel -->
            <ResizablePanel :default-size="35" :min-size="15">
                <EditorLogsPane />
            </ResizablePanel>
        </template>
    </ResizablePanelGroup>

    <!-- Close Confirmation Dialog -->
    <AlertDialog
        :open="closeConfirmDialogOpen"
        @update:open="closeConfirmDialogOpen = $event"
    >
        <AlertDialogContent>
            <AlertDialogHeader>
                <AlertDialogTitle>Unsaved Changes</AlertDialogTitle>
                <AlertDialogDescription>
                    This tab has unsaved changes. Are you sure you want to close
                    it? Your changes will be lost.
                </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction @click="handleCloseConfirm">
                    Close Tab
                </AlertDialogAction>
            </AlertDialogFooter>
        </AlertDialogContent>
    </AlertDialog>
</template>
