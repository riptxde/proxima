<script setup lang="ts">
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
</template>
