<script setup lang="ts">
import { Folder } from "@/components/ui/file-tree";
import {
    ContextMenu,
    ContextMenuContent,
    ContextMenuItem,
    ContextMenuSeparator,
    ContextMenuTrigger,
} from "@/components/ui/context-menu";
import { FolderOpenIcon, FileEditIcon, TrashIcon } from "lucide-vue-next";

defineProps<{
    id: string;
    name: string;
    path?: string;
}>();

const emit = defineEmits<{
    openLocation: [];
    rename: [];
    delete: [];
}>();
</script>

<template>
    <ContextMenu>
        <ContextMenuTrigger>
            <Folder :id="id" :name="name">
                <slot />
            </Folder>
        </ContextMenuTrigger>
        <ContextMenuContent class="w-48">
            <ContextMenuItem @click="emit('openLocation')">
                <FolderOpenIcon />
                Open Folder
            </ContextMenuItem>
            <ContextMenuSeparator />
            <ContextMenuItem @click="emit('rename')">
                <FileEditIcon />
                Rename
            </ContextMenuItem>
            <ContextMenuItem @click="emit('delete')" variant="destructive">
                <TrashIcon />
                Delete
            </ContextMenuItem>
        </ContextMenuContent>
    </ContextMenu>
</template>
