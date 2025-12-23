<script setup lang="ts">
import { ArrowUp, ArrowDown, Zap, FunctionSquare } from "lucide-vue-next";
import type { RemoteDirection, RemoteType } from "../types/remote-spy";

interface Props {
    activeDirections: RemoteDirection[];
    activeTypes: RemoteType[];
}

interface Emits {
    (e: "toggle-direction", direction: RemoteDirection): void;
    (e: "toggle-type", type: RemoteType): void;
}

defineProps<Props>();
defineEmits<Emits>();

const isDirectionActive = (
    direction: RemoteDirection,
    activeDirections: RemoteDirection[],
) => {
    return activeDirections.includes(direction);
};

const isTypeActive = (type: RemoteType, activeTypes: RemoteType[]) => {
    return activeTypes.includes(type);
};
</script>

<template>
    <div class="px-3 py-3 border-b border-border/60 bg-muted/40">
        <div class="flex items-center gap-4">
            <!-- Direction Filter Group -->
            <div class="flex-1">
                <div
                    class="text-[10px] font-medium text-muted-foreground uppercase tracking-wider mb-1.5 px-0.5"
                >
                    Direction
                </div>
                <div class="flex items-center gap-1">
                    <!-- Outgoing -->
                    <button
                        @click="$emit('toggle-direction', 'outgoing')"
                        class="relative flex items-center gap-1.5 h-7 px-2.5 rounded-md transition-all border"
                        :class="
                            isDirectionActive('outgoing', activeDirections)
                                ? 'bg-green-500/10 text-green-400 border-green-500/30'
                                : 'text-muted-foreground hover:text-foreground hover:bg-card border-border/50'
                        "
                    >
                        <ArrowUp class="w-3.5 h-3.5" />
                        <span class="text-xs font-medium">Out</span>
                    </button>

                    <!-- Incoming -->
                    <button
                        @click="$emit('toggle-direction', 'incoming')"
                        class="relative flex items-center gap-1.5 h-7 px-2.5 rounded-md transition-all border"
                        :class="
                            isDirectionActive('incoming', activeDirections)
                                ? 'bg-blue-500/10 text-blue-400 border-blue-500/30'
                                : 'text-muted-foreground hover:text-foreground hover:bg-card border-border/50'
                        "
                    >
                        <ArrowDown class="w-3.5 h-3.5" />
                        <span class="text-xs font-medium">In</span>
                    </button>
                </div>
            </div>

            <!-- Divider -->
            <div class="h-10 w-px bg-border/50" />

            <!-- Type Filter Group -->
            <div class="flex-1">
                <div
                    class="text-[10px] font-medium text-muted-foreground uppercase tracking-wider mb-1.5 px-0.5"
                >
                    Type
                </div>
                <div class="flex items-center gap-1">
                    <!-- RemoteEvent -->
                    <button
                        @click="$emit('toggle-type', 'RemoteEvent')"
                        class="relative flex items-center gap-1.5 h-7 px-2.5 rounded-md transition-all border"
                        :class="
                            isTypeActive('RemoteEvent', activeTypes)
                                ? 'bg-yellow-500/10 text-yellow-400 border-yellow-500/30'
                                : 'text-muted-foreground hover:text-foreground hover:bg-card border-border/50'
                        "
                    >
                        <Zap class="w-3.5 h-3.5" />
                        <span class="text-xs font-medium">Event</span>
                    </button>

                    <!-- RemoteFunction -->
                    <button
                        @click="$emit('toggle-type', 'RemoteFunction')"
                        class="relative flex items-center gap-1.5 h-7 px-2.5 rounded-md transition-all border"
                        :class="
                            isTypeActive('RemoteFunction', activeTypes)
                                ? 'bg-purple-500/10 text-purple-400 border-purple-500/30'
                                : 'text-muted-foreground hover:text-foreground hover:bg-card border-border/50'
                        "
                    >
                        <FunctionSquare class="w-3.5 h-3.5" />
                        <span class="text-xs font-medium">Function</span>
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
