<script setup lang="ts">
import { VueMonacoEditor } from "@guolao/vue-monaco-editor";
import type * as Monaco from "monaco-editor";
import { robloxDarkTheme } from "./themes";
import type { MonacoEditorOptions } from "./types";

interface Props {
    modelValue: string;
    language?: string;
    theme?: string;
    options?: MonacoEditorOptions;
}

const props = withDefaults(defineProps<Props>(), {
    language: "lua",
    theme: "roblox-dark",
    options: () => ({
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
        fontFamily: "Lilex, ui-monospace, monospace",
        lineNumbers: "on",
        roundedSelection: true,
        padding: {
            top: 0,
            bottom: 12,
        },
        overviewRulerLanes: 0,
        hideCursorInOverviewRuler: true,
        scrollBeyondLastLine: false,
        cursorSmoothCaretAnimation: "on",
        cursorBlinking: "smooth",
    }),
});

const emit = defineEmits<{
    "update:modelValue": [value: string];
    mount: [editor: Monaco.editor.IStandaloneCodeEditor, monaco: typeof Monaco];
}>();

const handleMount = (
    editor: Monaco.editor.IStandaloneCodeEditor,
    monaco: typeof Monaco,
) => {
    // Wait for fonts to load before measuring
    document.fonts.ready.then(() => {
        monaco.editor.remeasureFonts();
    });

    // Define and apply the Roblox Dark theme
    monaco.editor.defineTheme("roblox-dark", robloxDarkTheme);
    monaco.editor.setTheme(props.theme);

    // Emit mount event for parent components
    emit("mount", editor, monaco);
};
</script>

<template>
    <VueMonacoEditor
        :value="modelValue"
        @update:value="emit('update:modelValue', $event)"
        :theme="theme"
        :language="language"
        :options="options"
        @mount="handleMount"
        class="h-full w-full"
    />
</template>
