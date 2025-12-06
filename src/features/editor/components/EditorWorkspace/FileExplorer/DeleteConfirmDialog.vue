<script setup lang="ts">
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
} from "@/components/ui/alert-dialog";

defineProps<{
    open: boolean;
    itemName: string;
    type: "file" | "folder";
}>();

const emit = defineEmits<{
    "update:open": [value: boolean];
    confirm: [];
}>();

const handleConfirm = () => {
    emit("confirm");
    emit("update:open", false);
};
</script>

<template>
    <AlertDialog :open="open" @update:open="(val) => emit('update:open', val)">
        <AlertDialogContent>
            <AlertDialogHeader>
                <AlertDialogTitle>Are you sure?</AlertDialogTitle>
                <AlertDialogDescription>
                    This will permanently delete
                    <span class="font-semibold">{{ itemName }}</span
                    >{{ type === "folder" ? " and all its contents" : "" }}.
                    This action cannot be undone.
                </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction
                    @click="handleConfirm"
                    class="bg-destructive hover:bg-destructive/90"
                >
                    Delete
                </AlertDialogAction>
            </AlertDialogFooter>
        </AlertDialogContent>
    </AlertDialog>
</template>
