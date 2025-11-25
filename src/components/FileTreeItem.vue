<script setup lang="ts">
import { ChevronRight, ChevronDown, Folder, File } from "lucide-vue-next";

interface FileTreeItemProps {
    item: {
        name: string;
        type: "file" | "folder";
        expanded?: boolean;
        children?: any[];
    };
    level: number;
}

const props = defineProps<FileTreeItemProps>();
const emit = defineEmits<{
    toggle: [item: any];
}>();

const handleClick = () => {
    if (props.item.type === "folder") {
        emit("toggle", props.item);
    }
};
</script>

<template>
    <div>
        <div
            class="flex items-center gap-1 px-2 py-1 hover:bg-muted rounded cursor-pointer text-sm"
            :style="{ paddingLeft: `${level * 12 + 8}px` }"
            @click="handleClick"
        >
            <template v-if="item.type === 'folder'">
                <ChevronRight
                    v-if="!item.expanded"
                    :size="16"
                    class="text-muted-foreground flex-shrink-0"
                />
                <ChevronDown
                    v-else
                    :size="16"
                    class="text-muted-foreground flex-shrink-0"
                />
                <Folder
                    :size="16"
                    class="text-muted-foreground flex-shrink-0"
                />
            </template>
            <File
                v-else
                :size="16"
                class="text-muted-foreground flex-shrink-0"
            />
            <span class="truncate">{{ item.name }}</span>
        </div>

        <div v-if="item.type === 'folder' && item.expanded && item.children">
            <FileTreeItem
                v-for="(child, index) in item.children"
                :key="index"
                :item="child"
                :level="level + 1"
                @toggle="(childItem) => emit('toggle', childItem)"
            />
        </div>
    </div>
</template>
