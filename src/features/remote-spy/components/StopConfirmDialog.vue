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
} from '@/components/ui/alert-dialog';

defineProps<{
    open: boolean;
    callCount: number;
}>();

defineEmits<{
    'update:open': [value: boolean];
    confirm: [];
}>();
</script>

<template>
    <AlertDialog :open="open" @update:open="$emit('update:open', $event)">
        <AlertDialogContent>
            <AlertDialogHeader>
                <AlertDialogTitle>Stop Remote Spy</AlertDialogTitle>
                <AlertDialogDescription v-if="callCount > 0">
                    Stopping the remote spy will clear all {{ callCount }} remote call{{ callCount !== 1 ? 's' : '' }} from the list.
                    <br /><br />
                    If you want to temporarily stop receiving calls without clearing them, use the <strong>Pause</strong> button instead.
                </AlertDialogDescription>
                <AlertDialogDescription v-else>
                    Are you sure you want to stop the remote spy?
                </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction @click="$emit('confirm')">
                    Stop & Clear
                </AlertDialogAction>
            </AlertDialogFooter>
        </AlertDialogContent>
    </AlertDialog>
</template>
