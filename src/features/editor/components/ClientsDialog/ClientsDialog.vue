<script setup lang="ts">
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
} from "@/components/ui/dialog";
import ClientList from "./ClientList.vue";
import { useClients } from "@/features/editor/composables/useClients";

defineProps<{
    open: boolean;
}>();

defineEmits<{
    "update:open": [value: boolean];
}>();

const { clients } = useClients();
</script>

<template>
    <Dialog :open="open" @update:open="$emit('update:open', $event)">
        <DialogContent class="sm:max-w-[500px]">
            <DialogHeader>
                <DialogTitle>Clients</DialogTitle>
                <DialogDescription>
                    Select specific clients to execute scripts on.
                </DialogDescription>
            </DialogHeader>

            <div class="pt-4">
                <div
                    v-if="clients.length === 0"
                    class="text-center py-12 text-muted-foreground"
                >
                    <p class="text-sm">No clients attached</p>
                </div>

                <ClientList v-else />
            </div>
        </DialogContent>
    </Dialog>
</template>
