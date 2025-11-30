<script setup lang="ts">
import { X } from "lucide-vue-next";

defineEmits<{
    click: [];
}>();
</script>

<template>
    <button
        @click.stop="$emit('click')"
        class="hover:text-destructive transition-colors"
    >
        <X class="h-3.5 w-3.5" />
    </button>
</template>
