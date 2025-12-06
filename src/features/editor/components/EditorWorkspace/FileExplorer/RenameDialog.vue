<script setup lang="ts">
import { ref, watch } from "vue";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

const props = defineProps<{
    open: boolean;
    currentName: string;
    type: "file" | "folder";
}>();

const emit = defineEmits<{
    "update:open": [value: boolean];
    rename: [newName: string];
}>();

const newName = ref("");

// Reset input when dialog opens
watch(
    () => props.open,
    (isOpen) => {
        if (isOpen) {
            newName.value = props.currentName;
        }
    },
);

const handleRename = () => {
    if (newName.value.trim() && newName.value !== props.currentName) {
        emit("rename", newName.value.trim());
        emit("update:open", false);
    }
};

const handleCancel = () => {
    newName.value = "";
    emit("update:open", false);
};
</script>

<template>
    <Dialog :open="open" @update:open="(val) => emit('update:open', val)">
        <DialogContent class="sm:max-w-[425px]">
            <DialogHeader>
                <DialogTitle
                    >Rename
                    {{ type === "file" ? "File" : "Folder" }}</DialogTitle
                >
                <DialogDescription>
                    Enter a new name for this {{ type }}.
                </DialogDescription>
            </DialogHeader>
            <div class="grid gap-4 py-4">
                <div class="grid gap-2">
                    <label for="newName" class="text-sm font-medium">
                        {{ type === "file" ? "File" : "Folder" }} Name
                    </label>
                    <Input
                        id="newName"
                        v-model="newName"
                        :placeholder="currentName"
                        @keydown.enter="handleRename"
                    />
                </div>
            </div>
            <DialogFooter>
                <Button variant="outline" @click="handleCancel">
                    Cancel
                </Button>
                <Button
                    @click="handleRename"
                    :disabled="!newName.trim() || newName === currentName"
                >
                    Rename
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>
