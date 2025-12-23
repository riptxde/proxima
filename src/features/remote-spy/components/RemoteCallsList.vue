<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { Button } from "@/components/ui/button";
import {
    ArrowUp,
    ArrowDown,
    Zap,
    FunctionSquare,
    ChevronLeft,
    ChevronRight,
} from "lucide-vue-next";
import type { Remote } from "../types/remote-spy";

interface Props {
    remote: Remote;
    selectedCallId: string | null;
}

interface Emits {
    (e: "select-call", id: string): void;
    (e: "back"): void;
}

const props = defineProps<Props>();
defineEmits<Emits>();

// Pagination
const CALLS_PER_PAGE = 5;
const currentPage = ref(1);

const paginatedCalls = computed(() => {
    const start = (currentPage.value - 1) * CALLS_PER_PAGE;
    const end = start + CALLS_PER_PAGE;
    return props.remote.calls.slice(start, end);
});

const totalPages = computed(() => {
    return Math.ceil(props.remote.calls.length / CALLS_PER_PAGE);
});

const canGoToPrevPage = computed(() => currentPage.value > 1);
const canGoToNextPage = computed(() => currentPage.value < totalPages.value);

const goToPrevPage = () => {
    if (canGoToPrevPage.value) currentPage.value--;
};

const goToNextPage = () => {
    if (canGoToNextPage.value) currentPage.value++;
};

// Reset pagination when remote changes
watch(
    () => props.remote.id,
    () => {
        currentPage.value = 1;
    },
);

const formatTime = (date: Date) => {
    return (
        date.toLocaleTimeString("en-US", {
            hour12: false,
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit",
        }) +
        "." +
        String(date.getMilliseconds()).padStart(3, "0")
    );
};

const getDirectionIcon = (direction: "outgoing" | "incoming") => {
    return direction === "outgoing" ? ArrowUp : ArrowDown;
};

const getDirectionColor = (direction: "outgoing" | "incoming") => {
    return direction === "outgoing" ? "text-green-400" : "text-blue-400";
};

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
        <!-- Selected Remote Header -->
        <div class="px-3 py-3 border-b border-border/60 bg-muted/40 shrink-0">
            <!-- Back Button -->
            <Button
                variant="ghost"
                size="sm"
                class="mb-2 h-7 px-1! -ml-1 text-muted-foreground hover:text-foreground"
                @click="$emit('back')"
            >
                <ChevronLeft class="w-4 h-4" />
                <span class="text-xs">Back to Remotes</span>
            </Button>

            <!-- Remote Info -->
            <div class="flex items-center gap-2.5">
                <!-- Type Icon -->
                <component
                    :is="getTypeIcon(remote.type)"
                    class="w-4 h-4 shrink-0"
                    :class="getTypeColor(remote.type)"
                />

                <!-- Remote Name -->
                <span class="text-sm font-semibold truncate flex-1 min-w-0">{{
                    remote.name
                }}</span>

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
            </div>
        </div>

        <!-- Calls List -->
        <div class="flex-1 overflow-y-auto p-3">
            <div class="space-y-2">
                <div
                    v-for="call in paginatedCalls"
                    :key="call.id"
                    class="px-3 py-2.5 rounded-md transition-all border cursor-pointer"
                    :class="
                        selectedCallId === call.id
                            ? 'bg-blue-500/30 shadow-[0_0_0_1px_rgb(70_150_250/0.5)] hover:bg-blue-500/40 border-blue-500/50'
                            : 'bg-muted/30 border-border/50 hover:bg-muted/50 hover:border-border'
                    "
                    @click="$emit('select-call', call.id)"
                >
                    <div class="flex items-center gap-2.5">
                        <!-- Direction Icon -->
                        <component
                            :is="getDirectionIcon(call.direction)"
                            class="w-4 h-4 shrink-0"
                            :class="getDirectionColor(call.direction)"
                        />

                        <!-- Call Info -->
                        <div class="flex-1 min-w-0 space-y-1">
                            <div class="text-xs text-foreground/90 font-mono">
                                {{ formatTime(call.timestamp) }}
                            </div>
                            <div
                                v-if="call.arguments.length > 0"
                                class="text-[11px] text-muted-foreground/80 font-mono truncate"
                            >
                                {{
                                    call.arguments
                                        .map((a) => a.value)
                                        .join(", ")
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Pagination Controls -->
        <div
            v-if="totalPages > 1"
            class="px-3 py-2.5 border-t border-border/60 bg-muted/40 shrink-0 flex items-center justify-between"
        >
            <Button
                variant="ghost"
                size="sm"
                class="h-7 px-1!"
                :disabled="!canGoToPrevPage"
                @click="goToPrevPage"
            >
                <ChevronLeft class="w-3.5 h-3.5" />
                <span class="text-xs">Prev</span>
            </Button>

            <span class="text-xs text-muted-foreground font-mono">
                Page {{ currentPage }} of {{ totalPages }}
            </span>

            <Button
                variant="ghost"
                size="sm"
                class="h-7 px-1!"
                :disabled="!canGoToNextPage"
                @click="goToNextPage"
            >
                <span class="text-xs">Next</span>
                <ChevronRight class="w-3.5 h-3.5" />
            </Button>
        </div>
    </div>
</template>
