<script setup lang="ts">
import type { Component } from "vue";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip";

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
    <TooltipProvider>
        <Tooltip>
            <TooltipTrigger as-child>
                <button
                    @click="$emit('click')"
                    :class="[
                        'h-12 w-12 flex items-center justify-center rounded-lg transition-all duration-300 relative z-10',
                        isActive
                            ? 'text-app-shell-accent-foreground'
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
    </TooltipProvider>
</template>
