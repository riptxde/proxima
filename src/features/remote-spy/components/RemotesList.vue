<script setup lang="ts">
import { Input } from "@/components/ui/input";
import {
    Search,
    Zap,
    FunctionSquare,
    ArrowUp,
    ArrowDown,
    ChevronRight,
} from "lucide-vue-next";
import type { Remote } from "../types/remote-spy";

interface Props {
    remotes: Remote[];
    searchQuery: string;
}

interface Emits {
    (e: "select", id: string): void;
    (e: "update:search-query", value: string): void;
}

defineProps<Props>();
defineEmits<Emits>();

const getTypeIcon = (type: "RemoteEvent" | "RemoteFunction") => {
    return type === "RemoteEvent" ? Zap : FunctionSquare;
};

const getTypeColor = (type: "RemoteEvent" | "RemoteFunction") => {
    return type === "RemoteEvent" ? "text-yellow-400" : "text-purple-400";
};

const getDirectionStats = (
    calls: Array<{ direction: "outgoing" | "incoming" }>,
) => {
    const outgoing = calls.filter((c) => c.direction === "outgoing").length;
    const incoming = calls.filter((c) => c.direction === "incoming").length;
    return { outgoing, incoming };
};
</script>

<template>
    <div class="flex-1 overflow-hidden flex flex-col">
        <!-- Search Bar -->
        <div class="px-3 pt-3 pb-2 shrink-0">
            <div class="relative">
                <Search
                    class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground"
                />
                <Input
                    :model-value="searchQuery"
                    @update:model-value="
                        (val) => $emit('update:search-query', String(val))
                    "
                    placeholder="Search"
                    class="pl-9 h-10"
                />
            </div>
        </div>

        <!-- Remotes List or Empty State -->
        <div class="flex-1 overflow-y-auto px-3 pb-3">
            <!-- Empty State -->
            <div
                v-if="remotes.length === 0"
                class="h-full flex items-center justify-center text-center py-12"
            >
                <div class="text-muted-foreground">
                    <Zap class="w-12 h-12 mx-auto mb-3 opacity-30" />
                    <p class="text-sm">No remote calls captured</p>
                    <p class="text-xs mt-1">
                        Start the spy to begin monitoring
                    </p>
                </div>
            </div>

            <!-- Remotes List -->
            <div v-else class="space-y-2">
                <div
                    v-for="remote in remotes"
                    :key="remote.id"
                    class="px-3 py-2.5 rounded-md transition-all border border-border/50 bg-muted/30 hover:bg-muted/50 hover:border-border cursor-pointer"
                    @click="$emit('select', remote.id)"
                >
                    <div class="flex items-center gap-2.5">
                        <!-- Type Icon -->
                        <component
                            :is="getTypeIcon(remote.type)"
                            class="w-4 h-4 shrink-0"
                            :class="getTypeColor(remote.type)"
                        />

                        <!-- Remote Name -->
                        <span
                            class="text-sm font-semibold truncate flex-1 min-w-0"
                            >{{ remote.name }}</span
                        >

                        <!-- Direction Stats -->
                        <div
                            v-if="getDirectionStats(remote.calls).outgoing > 0"
                            class="flex items-center gap-1 text-green-400/80"
                        >
                            <ArrowUp class="w-3 h-3" />
                            <span class="font-mono text-[11px]">{{
                                getDirectionStats(remote.calls).outgoing
                            }}</span>
                        </div>
                        <div
                            v-if="getDirectionStats(remote.calls).incoming > 0"
                            class="flex items-center gap-1 text-blue-400/80"
                        >
                            <ArrowDown class="w-3 h-3" />
                            <span class="font-mono text-[11px]">{{
                                getDirectionStats(remote.calls).incoming
                            }}</span>
                        </div>

                        <!-- Chevron -->
                        <ChevronRight
                            class="w-4 h-4 text-muted-foreground shrink-0"
                        />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
