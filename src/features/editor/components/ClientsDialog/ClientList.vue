<script setup lang="ts">
import { computed } from "vue";
import { Checkbox } from "@/components/ui/checkbox";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { useClients } from "@/features/editor/composables/useClients";

const { clients, isClientSelected, toggleClient, selectAll, deselectAll } =
    useClients();

const allSelected = computed(() => {
    return (
        clients.value.length > 0 &&
        clients.value.every((c) => isClientSelected(c.id))
    );
});

const someSelected = computed(() => {
    const selectedCount = clients.value.filter((c) =>
        isClientSelected(c.id),
    ).length;
    return selectedCount > 0 && selectedCount < clients.value.length;
});

const toggleAll = () => {
    if (allSelected.value) {
        deselectAll();
    } else {
        selectAll();
    }
};
</script>

<template>
    <div class="rounded-md border overflow-hidden">
        <div class="max-h-[450px] overflow-y-auto">
            <Table>
                <TableHeader class="sticky top-0 bg-card z-10 border-b">
                    <TableRow>
                        <TableHead class="w-12">
                            <Checkbox
                                :model-value="allSelected"
                                :indeterminate="someSelected"
                                @update:model-value="toggleAll"
                            />
                        </TableHead>
                        <TableHead>Client Name</TableHead>
                        <TableHead>Client ID</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow
                        v-for="client in clients"
                        :key="client.id"
                        class="cursor-pointer"
                        @click="toggleClient(client.id)"
                    >
                        <TableCell class="w-12">
                            <Checkbox
                                :model-value="isClientSelected(client.id)"
                                @click.stop
                                @update:model-value="toggleClient(client.id)"
                            />
                        </TableCell>
                        <TableCell class="font-medium">{{
                            client.username
                        }}</TableCell>
                        <TableCell
                            class="font-mono text-xs text-muted-foreground"
                        >
                            {{ client.id }}
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>
    </div>
</template>
