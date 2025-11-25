<script setup lang="ts">
import { X, Minus, Square } from "lucide-vue-next";
</script>

<template>
    <div
        class="h-10 bg-muted/50 border-b flex items-center justify-between px-4"
    >
        <div class="text-sm font-medium">Riptide</div>
        <div class="flex items-center gap-2">
            <button
                class="h-6 w-6 flex items-center justify-center hover:bg-muted rounded transition-colors"
            >
                <Minus :size="16" />
            </button>
            <button
                class="h-6 w-6 flex items-center justify-center hover:bg-muted rounded transition-colors"
            >
                <Square :size="14" />
            </button>
            <button
                class="h-6 w-6 flex items-center justify-center hover:bg-destructive hover:text-destructive-foreground rounded transition-colors"
            >
                <X :size="16" />
            </button>
        </div>
    </div>
</template>
