<script setup lang="ts">
import { ref } from "vue";

// Mock logs data
const logs = ref(["Welcome to Riptide", "Ready to execute scripts..."]);
</script>

<template>
    <div class="h-full flex flex-col bg-muted/20">
        <div class="px-4 py-2 border-b">
            <h3 class="text-sm font-semibold">Logs</h3>
        </div>
        <div class="flex-1 overflow-auto p-4 font-mono text-sm">
            <div
                v-for="(log, index) in logs"
                :key="index"
                class="text-muted-foreground mb-1"
            >
                {{ log }}
            </div>
        </div>
    </div>
</template>
