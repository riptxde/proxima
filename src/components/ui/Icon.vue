<template>
    <component :is="iconComponent" :size="size" />
</template>

<script setup lang="ts">
import { computed } from "vue";
import { FolderOpen, Folder, Code } from "lucide-vue-next";

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
        "lucide:folder-open": FolderOpen,
        "lucide:folder": Folder,
        "lucide:code": Code,
    };

    return iconMap[props.name] || Code;
});
</script>
