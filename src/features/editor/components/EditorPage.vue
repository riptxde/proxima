<script setup lang="ts">
import StarsBackground from "@/components/ui/bg-stars/StarsBackground.vue";
import EditorWorkspace from "./EditorWorkspace/EditorWorkspace.vue";
import EditorDock from "./EditorDock/EditorDock.vue";
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
            <EditorWorkspace />
            <EditorDock />
        </div>
    </div>
</template>
