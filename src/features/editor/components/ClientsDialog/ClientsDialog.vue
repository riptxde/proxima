<script setup lang="ts">
import { ref, computed } from "vue";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import ClientList from "./ClientList.vue";
import { useExecutorClients } from "@/features/editor/composables/useExecutorClients";
import { Search } from "lucide-vue-next";

defineProps<{
    open: boolean;
}>();

defineEmits<{
    "update:open": [value: boolean];
}>();

const { clients } = useExecutorClients();
const searchQuery = ref("");

const filteredClients = computed(() => {
    if (!searchQuery.value.trim()) {
        return clients.value;
    }

    const query = searchQuery.value.toLowerCase();
    return clients.value.filter((client) =>
        client.username.toLowerCase().includes(query),
    );
});
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

            <div class="space-y-4">
                <div class="relative">
                    <Search
                        class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground"
                    />
                    <Input
                        v-model="searchQuery"
                        placeholder="Search"
                        class="pl-9"
                    />
                </div>

                <div
                    v-if="clients.length === 0"
                    class="text-center py-12 text-muted-foreground"
                >
                    <p class="text-sm">No clients attached</p>
                </div>

                <div
                    v-else-if="filteredClients.length === 0"
                    class="text-center py-12 text-muted-foreground"
                >
                    <p class="text-sm">No clients match your search</p>
                </div>

                <ClientList v-else :clients="filteredClients" />
            </div>
        </DialogContent>
    </Dialog>
</template>
