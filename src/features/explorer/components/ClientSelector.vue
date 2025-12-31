<script setup lang="ts">
import { ref, computed } from "vue";
import { Button } from "@/components/ui/button";
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover";
import { Input } from "@/components/ui/input";
import { ChevronDown, Search } from "lucide-vue-next";
import type { Client } from "@/types/client";

interface Props {
    selectedClient: Client | null;
    clients: Client[];
}

interface Emits {
    (e: "select", client: Client): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const searchQuery = ref("");
const open = ref(false);

const filteredClients = computed(() => {
    if (!searchQuery.value) return props.clients;
    return props.clients.filter((client) =>
        client.username.toLowerCase().includes(searchQuery.value.toLowerCase()),
    );
});

const selectClient = (client: Client) => {
    emit("select", client);
    open.value = false;
    searchQuery.value = "";
};
</script>

<template>
    <Popover v-model:open="open">
        <PopoverTrigger as-child>
            <Button
                variant="outline"
                class="w-full justify-between bg-card/50 hover:bg-card/70 border-border/50"
            >
                <span class="truncate">
                    {{
                        selectedClient
                            ? selectedClient.username
                            : "Select a client..."
                    }}
                </span>
                <ChevronDown class="ml-2 h-4 w-4 opacity-50" />
            </Button>
        </PopoverTrigger>
        <PopoverContent class="w-[300px] p-0 bg-card border-border/50">
            <div class="p-3 border-b border-border/50">
                <div class="relative">
                    <Search
                        class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground"
                    />
                    <Input
                        v-model="searchQuery"
                        placeholder="Search clients..."
                        class="pl-8 bg-background/50 border-border/50"
                    />
                </div>
            </div>
            <div class="max-h-[300px] overflow-y-auto">
                <div
                    v-for="client in filteredClients"
                    :key="client.id"
                    class="px-3 py-2 cursor-pointer hover:bg-accent/50 transition-colors"
                    :class="{
                        'bg-accent/30': selectedClient?.id === client.id,
                    }"
                    @click="selectClient(client)"
                >
                    <div class="flex items-center gap-2">
                        <div
                            class="w-2 h-2 rounded-full bg-green-500"
                            :class="{
                                'bg-green-500': true,
                            }"
                        />
                        <span class="text-sm">{{ client.username }}</span>
                    </div>
                </div>
                <div
                    v-if="filteredClients.length === 0"
                    class="px-3 py-8 text-center text-sm text-muted-foreground"
                >
                    No clients found
                </div>
            </div>
        </PopoverContent>
    </Popover>
</template>
