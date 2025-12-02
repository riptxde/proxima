<template>
    <component :is="iconComponent" :size="size" />
</template>

<script setup lang="ts">
import { computed } from "vue";
import { FolderOpen, Folder, Scroll } from "lucide-vue-next";

interface IconProps {
    name: string;
    size?: string | number;
}

const props = withDefaults(defineProps<IconProps>(), {
    size: 16,
});

const iconComponent = computed(() => {
    // Map icon names to lucide-vue-next components
    const iconMap: Record<string, any> = {
        "folder-open": FolderOpen,
        folder: Folder,
        file: Scroll,
    };

    return iconMap[props.name] || Scroll;
});
</script>
