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

const { clients, isClientEnabled, toggleClient, enableAll, disableAll } =
    useClients();

const allEnabled = computed(() => {
    return (
        clients.value.length > 0 &&
        clients.value.every((c) => isClientEnabled(c.id))
    );
});

const someEnabled = computed(() => {
    const enabledCount = clients.value.filter((c) =>
        isClientEnabled(c.id),
    ).length;
    return enabledCount > 0 && enabledCount < clients.value.length;
});

const toggleAll = () => {
    if (allEnabled.value) {
        disableAll();
    } else {
        enableAll();
    }
};
</script>

<template>
    <div class="rounded-md border">
        <Table>
            <TableHeader class="sticky top-0 bg-background z-10">
                <TableRow>
                    <TableHead class="w-12">
                        <Checkbox
                            :model-value="allEnabled"
                            :indeterminate="someEnabled"
                            @update:model-value="toggleAll"
                        />
                    </TableHead>
                    <TableHead>Client Name</TableHead>
                    <TableHead>Client ID</TableHead>
                </TableRow>
            </TableHeader>
        </Table>
        <div class="max-h-[400px] overflow-y-auto">
            <Table>
                <TableBody>
                    <TableRow
                        v-for="client in clients"
                        :key="client.id"
                        class="cursor-pointer"
                        @click="toggleClient(client.id)"
                    >
                        <TableCell class="w-12">
                            <Checkbox
                                :model-value="isClientEnabled(client.id)"
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
