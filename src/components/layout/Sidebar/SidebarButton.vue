<script setup lang="ts">
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import type { Component } from "vue";

interface Props {
    icon: Component;
    label: string;
    isActive: boolean;
}

defineProps<Props>();

defineEmits<{
    click: [];
}>();
</script>

<template>
    <Tooltip>
        <TooltipTrigger as-child>
            <button
                @click="$emit('click')"
                :class="[
                    'h-12 w-12 flex items-center justify-center rounded-lg transition-opacity',
                    isActive
                        ? 'bg-app-shell-accent text-app-shell-accent-foreground'
                        : 'text-app-shell-foreground opacity-60 hover:opacity-100',
                ]"
            >
                <component :is="icon" :size="20" />
            </button>
        </TooltipTrigger>
        <TooltipContent side="right">
            <p>{{ label }}</p>
        </TooltipContent>
    </Tooltip>
</template>
