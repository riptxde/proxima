<script setup lang="ts">
import { ref } from "vue";
import { VueMonacoEditor } from "@guolao/vue-monaco-editor";
import { Card } from "@/components/ui/card";

const scriptContent = ref("-- Write your script here...");

const editorOptions = {
    automaticLayout: true,
    formatOnType: true,
    formatOnPaste: true,
    minimap: {
        enabled: false,
    },
    scrollbar: {
        verticalScrollbarSize: 12,
        horizontalScrollbarSize: 12,
    },
    fontSize: 14,
    fontFamily:
        "Geist Mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
    lineNumbers: "on",
    roundedSelection: false,
    padding: {
        top: 12,
        bottom: 12,
    },
    overviewRulerLanes: 0,
    hideCursorInOverviewRuler: true,
    scrollBeyondLastLine: false,
};
</script>

<template>
    <Card class="h-full overflow-hidden p-0">
        <VueMonacoEditor
            v-model:value="scriptContent"
            theme="vs-dark"
            language="lua"
            :options="editorOptions"
            class="h-full w-full"
        />
    </Card>
</template>
