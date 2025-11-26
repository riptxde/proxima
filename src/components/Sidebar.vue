<script setup lang="ts">
import { FileCode2, Library, Settings, Terminal } from "lucide-vue-next";
</script>

<template>
    <div
        class="w-16 bg-sidebar border-r border-sidebar-border flex flex-col items-center py-4 gap-2"
    >
        <button
            class="h-12 w-12 flex items-center justify-center rounded-lg bg-sidebar-accent text-sidebar-accent-foreground transition-colors"
            title="Editor"
        >
            <FileCode2 :size="20" />
        </button>
        <button
            class="h-12 w-12 flex items-center justify-center rounded-lg hover:bg-sidebar-accent hover:text-sidebar-accent-foreground text-sidebar-foreground/60 transition-colors"
            title="Script Hub"
        >
            <Library :size="20" />
        </button>
        <button
            class="h-12 w-12 flex items-center justify-center rounded-lg hover:bg-sidebar-accent hover:text-sidebar-accent-foreground text-sidebar-foreground/60 transition-colors"
            title="Logs"
        >
            <Terminal :size="20" />
        </button>
        <button
            class="h-12 w-12 flex items-center justify-center rounded-lg hover:bg-sidebar-accent hover:text-sidebar-accent-foreground text-sidebar-foreground/60 transition-colors"
            title="Settings"
        >
            <Settings :size="20" />
        </button>
    </div>
</template>
