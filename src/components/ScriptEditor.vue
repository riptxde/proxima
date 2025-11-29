<script setup lang="ts">
import { ref, computed, nextTick } from "vue";
import { VueMonacoEditor } from "@guolao/vue-monaco-editor";
import { Card } from "@/components/ui/card";
import { X, Plus } from "lucide-vue-next";
import type * as Monaco from "monaco-editor";

interface Tab {
    id: number;
    name: string;
    content: string;
}

const tabs = ref<Tab[]>([
    { id: 1, name: "Script 1", content: "-- Write your script here..." },
]);

const activeTabId = ref(1);
const nextTabId = ref(2);
const editingTabId = ref<number | null>(null);
const editingTabName = ref("");
const inputWidth = ref(0);

const activeTab = computed(() =>
    tabs.value.find((tab) => tab.id === activeTabId.value),
);

const scriptContent = computed({
    get: () => activeTab.value?.content || "",
    set: (value: string) => {
        const tab = activeTab.value;
        if (tab) {
            tab.content = value;
        }
    },
});

const addTab = () => {
    const newTab: Tab = {
        id: nextTabId.value,
        name: `Script ${nextTabId.value}`,
        content: "-- Write your script here...",
    };
    nextTabId.value++;
    tabs.value.push(newTab);
    activeTabId.value = newTab.id;
};

const closeTab = (tabId: number) => {
    if (tabs.value.length === 1) return;

    const index = tabs.value.findIndex((tab) => tab.id === tabId);
    if (index === -1) return;

    tabs.value.splice(index, 1);

    if (activeTabId.value === tabId) {
        const newIndex = Math.max(0, index - 1);
        activeTabId.value = tabs.value[newIndex].id;
    }
};

const selectTab = (tabId: number) => {
    if (editingTabId.value === null) {
        activeTabId.value = tabId;
    }
};

const startRenaming = async (tabId: number, event?: MouseEvent) => {
    if (activeTabId.value !== tabId) return;

    const tab = tabs.value.find((t) => t.id === tabId);
    if (!tab) return;

    // Get the actual rendered width of the span
    const span = event?.target as HTMLElement;
    if (span && span.tagName === "SPAN") {
        inputWidth.value = Math.min(span.offsetWidth, 96);
    } else {
        inputWidth.value = 96;
    }

    editingTabId.value = tabId;
    editingTabName.value = tab.name;

    await nextTick();
    const input = document.querySelector(
        `input[data-tab-id="${tabId}"]`,
    ) as HTMLInputElement;
    input?.select();
};

const measureTextWidth = (text: string) => {
    const canvas = document.createElement("canvas");
    const context = canvas.getContext("2d");
    if (!context) return 0;
    context.font = "14px ui-sans-serif, system-ui, sans-serif";
    return Math.min(context.measureText(text).width + 4, 96);
};

const updateInputWidth = () => {
    inputWidth.value = measureTextWidth(editingTabName.value);
};

const finishRenaming = () => {
    if (editingTabId.value === null) return;

    const tab = tabs.value.find((t) => t.id === editingTabId.value);
    if (tab && editingTabName.value.trim()) {
        tab.name = editingTabName.value.trim();
    }

    editingTabId.value = null;
};

const cancelRenaming = () => {
    editingTabId.value = null;
};

const editorOptions = {
    automaticLayout: true,
    formatOnType: false,
    formatOnPaste: false,
    minimap: {
        enabled: false,
    },
    scrollbar: {
        verticalScrollbarSize: 12,
        horizontalScrollbarSize: 12,
    },
    fontSize: 14,
    fontFamily: "Cascadia Code, ui-monospace, monospace",
    lineNumbers: "on",
    roundedSelection: true,
    padding: {
        top: 0,
        bottom: 12,
    },
    overviewRulerLanes: 0,
    hideCursorInOverviewRuler: true,
    scrollBeyondLastLine: false,
};

const handleMount = (
    _editor: Monaco.editor.IStandaloneCodeEditor,
    monaco: typeof Monaco,
) => {
    // Define the Roblox Dark theme
    monaco.editor.defineTheme("roblox-dark", {
        base: "vs-dark",
        inherit: true,
        rules: [
            // Comments
            { token: "comment", foreground: "666666", fontStyle: "italic" },
            {
                token: "comment.line",
                foreground: "666666",
                fontStyle: "italic",
            },
            {
                token: "comment.block",
                foreground: "666666",
                fontStyle: "italic",
            },

            // Variables
            { token: "variable", foreground: "F86D7C" },
            { token: "variable.name", foreground: "F86D7C" },
            { token: "variable.other", foreground: "F86D7C" },

            // Keywords and control flow
            { token: "keyword", foreground: "F86D7C" },
            { token: "keyword.control", foreground: "F86D7C" },
            { token: "keyword.operator", foreground: "cccccc" },
            { token: "keyword.operator.logical", foreground: "cccccc" },

            // Storage types
            { token: "storage.type", foreground: "cccccc" },
            { token: "storage.modifier", foreground: "cccccc" },

            // Functions
            { token: "entity.name.function", foreground: "84D6F7" },
            { token: "support.function", foreground: "84D6F7" },
            { token: "meta.function-call", foreground: "84D6F7" },

            // Constants and numbers
            { token: "constant", foreground: "FFC600" },
            { token: "constant.numeric", foreground: "FFC600" },
            { token: "constant.language", foreground: "f07178" },
            { token: "constant.character", foreground: "FFC600" },
            { token: "constant.escape", foreground: "FFC600" },

            // Strings
            { token: "string", foreground: "ADF195" },
            { token: "string.quoted", foreground: "ADF195" },

            // Classes and types
            { token: "entity.name.class", foreground: "FFCB6B" },
            { token: "entity.name.type", foreground: "FFCB6B" },
            { token: "support.type", foreground: "F86D7C" },
            { token: "support.class", foreground: "FFCB6B" },

            // Punctuation
            { token: "punctuation", foreground: "cccccc" },
            { token: "punctuation.definition.string", foreground: "ADF195" },

            // Invalid
            { token: "invalid", foreground: "FF0000" },
            { token: "invalid.illegal", foreground: "FF0000" },
        ],
        colors: {
            // Use card background to match file explorer panel
            "editor.background": "#171717",
            "editor.foreground": "#d4e4e4",

            // Selection and highlights
            "editor.lineHighlightBackground": "#2a2a2a",
            "editor.selectionBackground": "#3a3a3a",
            "editor.inactiveSelectionBackground": "#2f2f2f",

            // Gutter
            "editorLineNumber.foreground": "#666666",
            "editorLineNumber.activeForeground": "#bbbbbb",
            "editorGutter.background": "#171717",

            // Cursor
            "editorCursor.foreground": "#eeffff",

            // Scrollbar
            "scrollbarSlider.background": "#2E2E2E",
            "scrollbarSlider.hoverBackground": "#3a3a3a",
            "scrollbarSlider.activeBackground": "#454545",

            // Borders
            "editorWidget.border": "#222222",

            // Suggestions/autocomplete
            "editorSuggestWidget.background": "#272727",
            "editorSuggestWidget.border": "#222222",
            "editorSuggestWidget.selectedBackground": "#232323",

            // Find/replace widget
            "editorWidget.background": "#272727",
        },
    });

    // Apply the theme
    monaco.editor.setTheme("roblox-dark");
};
</script>

<template>
    <Card class="h-full overflow-hidden p-2 flex flex-col gap-3">
        <!-- Tab Bar -->
        <div
            class="flex items-center gap-1 bg-input/30 rounded-md p-1.5 overflow-x-auto"
        >
            <button
                v-for="tab in tabs"
                :key="tab.id"
                @click="selectTab(tab.id)"
                :class="[
                    'flex items-center gap-2 px-3 py-1.5 rounded transition-all duration-200 border',
                    activeTabId === tab.id
                        ? 'border-border text-foreground'
                        : 'border-transparent text-muted-foreground hover:text-foreground',
                ]"
            >
                <input
                    v-if="editingTabId === tab.id"
                    v-model="editingTabName"
                    @input="updateInputWidth"
                    @blur="finishRenaming"
                    @keydown.enter="finishRenaming"
                    @keydown.esc="cancelRenaming"
                    @click.stop
                    :data-tab-id="tab.id"
                    :style="{ width: inputWidth + 'px' }"
                    class="text-sm bg-transparent border-none outline-none"
                />
                <span
                    v-else
                    @dblclick="startRenaming(tab.id, $event)"
                    class="text-sm truncate max-w-24"
                >
                    {{ tab.name }}
                </span>
                <button
                    v-if="tabs.length > 1"
                    @click.stop="closeTab(tab.id)"
                    class="hover:text-destructive transition-colors"
                >
                    <X class="h-3.5 w-3.5" />
                </button>
            </button>

            <button
                @click="addTab"
                class="flex items-center justify-center w-7 h-7 rounded text-muted-foreground hover:text-foreground hover:bg-accent transition-all"
                title="New Tab"
            >
                <Plus class="h-4 w-4" />
            </button>
        </div>

        <!-- Editor -->
        <div class="flex-1 overflow-hidden">
            <VueMonacoEditor
                v-model:value="scriptContent"
                theme="roblox-dark"
                language="lua"
                :options="editorOptions"
                @mount="handleMount"
                class="h-full w-full"
            />
        </div>
    </Card>
</template>
