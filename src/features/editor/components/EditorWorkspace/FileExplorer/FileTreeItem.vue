<script setup lang="ts">
import { File } from "@/components/ui/file-tree";
import {
    ContextMenu,
    ContextMenuContent,
    ContextMenuItem,
    ContextMenuSeparator,
    ContextMenuTrigger,
} from "@/components/ui/context-menu";
import { FolderOpen, Pencil, Trash } from "lucide-vue-next";

defineProps<{
    id: string;
    name: string;
    path?: string;
}>();

const emit = defineEmits<{
    click: [];
    openLocation: [];
    rename: [];
    delete: [];
}>();
</script>

<template>
    <ContextMenu>
        <ContextMenuTrigger>
            <File :id="id" :name="name" @click="emit('click')" />
        </ContextMenuTrigger>
        <ContextMenuContent class="w-48">
            <ContextMenuItem @click="emit('openLocation')">
                <FolderOpen />
                Open File Location
            </ContextMenuItem>
            <ContextMenuSeparator />
            <ContextMenuItem @click="emit('rename')">
                <Pencil />
                Rename
            </ContextMenuItem>
            <ContextMenuItem @click="emit('delete')" variant="destructive">
                <Trash />
                Delete
            </ContextMenuItem>
        </ContextMenuContent>
    </ContextMenu>
</template>
