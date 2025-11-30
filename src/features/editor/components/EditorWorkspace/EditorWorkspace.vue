<script setup lang="ts">
import {
    ResizablePanelGroup,
    ResizablePanel,
    ResizableHandle,
} from "@/components/ui/resizable";
import CodeEditor from "./CodeEditor/CodeEditor.vue";
import FileExplorer from "./FileExplorer/FileExplorer.vue";
</script>

<template>
    <div class="flex-1 overflow-hidden pt-4 px-4">
        <ResizablePanelGroup direction="horizontal" class="h-full gap-4">
            <!-- Script Editor -->
            <ResizablePanel :default-size="70" :min-size="30">
                <CodeEditor />
            </ResizablePanel>

            <ResizableHandle with-handle />

            <!-- File Tree Panel -->
            <ResizablePanel :default-size="25" :min-size="20">
                <FileExplorer />
            </ResizablePanel>
        </ResizablePanelGroup>
    </div>
</template>
