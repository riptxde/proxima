<script setup lang="ts">
import { Plus } from "lucide-vue-next";

defineEmits<{
    click: [];
}>();
</script>

<template>
    <button
        @click="$emit('click')"
        class="flex items-center justify-center w-7 h-7 rounded text-muted-foreground hover:text-foreground hover:bg-accent transition-all"
    >
        <Plus class="h-4 w-4" />
    </button>
</template>
