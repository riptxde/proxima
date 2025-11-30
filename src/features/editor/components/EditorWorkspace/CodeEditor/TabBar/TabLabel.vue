<script setup lang="ts">
interface Props {
    name: string;
}

defineProps<Props>();

defineEmits<{
    dblclick: [];
}>();
</script>

<template>
    <span @dblclick="$emit('dblclick')" class="text-sm truncate max-w-24">
        {{ name }}
    </span>
</template>
