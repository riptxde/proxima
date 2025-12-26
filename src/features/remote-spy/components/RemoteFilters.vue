<script setup lang="ts">
import {
    ArrowUp,
    ArrowDown,
    Zap,
    FunctionSquare,
    Radio,
} from "lucide-vue-next";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import type { RemoteDirection, RemoteClass } from "../types/remote-spy";

interface Props {
    activeDirections: RemoteDirection[];
    activeClasses: RemoteClass[];
}

interface Emits {
    (e: "toggle-direction", direction: RemoteDirection): void;
    (e: "toggle-class", remoteClass: RemoteClass): void;
}

defineProps<Props>();
defineEmits<Emits>();

const isDirectionActive = (
    direction: RemoteDirection,
    activeDirections: RemoteDirection[],
) => {
    return activeDirections.includes(direction);
};

const isClassActive = (
    remoteClass: RemoteClass,
    activeClasses: RemoteClass[],
) => {
    return activeClasses.includes(remoteClass);
};
</script>

<template>
    <TooltipProvider :delay-duration="200">
        <div class="px-3 py-3 border-b border-border/60 bg-muted/40">
            <div class="flex items-center justify-between">
                <!-- Direction Filter Group -->
                <div class="flex items-center gap-2.5">
                    <div
                        class="text-[11px] font-medium text-muted-foreground uppercase tracking-wide"
                    >
                        Direction
                    </div>
                    <div class="flex items-center gap-1">
                        <Tooltip>
                            <TooltipTrigger as-child>
                                <button
                                    @click="
                                        $emit('toggle-direction', 'outgoing')
                                    "
                                    class="relative flex items-center justify-center w-7 h-7 rounded-md transition-all border"
                                    :class="
                                        isDirectionActive(
                                            'outgoing',
                                            activeDirections,
                                        )
                                            ? 'bg-green-500/10 text-green-400 border-green-500/30'
                                            : 'text-muted-foreground hover:text-foreground hover:bg-card border-border/50'
                                    "
                                >
                                    <ArrowUp class="w-3.5 h-3.5" />
                                </button>
                            </TooltipTrigger>
                            <TooltipContent side="bottom">
                                <p class="text-xs">Outgoing</p>
                            </TooltipContent>
                        </Tooltip>

                        <Tooltip>
                            <TooltipTrigger as-child>
                                <button
                                    @click="
                                        $emit('toggle-direction', 'incoming')
                                    "
                                    class="relative flex items-center justify-center w-7 h-7 rounded-md transition-all border"
                                    :class="
                                        isDirectionActive(
                                            'incoming',
                                            activeDirections,
                                        )
                                            ? 'bg-blue-500/10 text-blue-400 border-blue-500/30'
                                            : 'text-muted-foreground hover:text-foreground hover:bg-card border-border/50'
                                    "
                                >
                                    <ArrowDown class="w-3.5 h-3.5" />
                                </button>
                            </TooltipTrigger>
                            <TooltipContent side="bottom">
                                <p class="text-xs">Incoming</p>
                            </TooltipContent>
                        </Tooltip>
                    </div>
                </div>

                <!-- Divider -->
                <div class="h-7 w-px bg-border/50" />

                <!-- Class Filter Group -->
                <div class="flex items-center gap-2.5">
                    <div
                        class="text-xs font-medium text-muted-foreground uppercase tracking-wide"
                    >
                        Class
                    </div>
                    <div class="flex items-center gap-1">
                        <Tooltip>
                            <TooltipTrigger as-child>
                                <button
                                    @click="
                                        $emit('toggle-class', 'RemoteEvent')
                                    "
                                    class="relative flex items-center justify-center w-7 h-7 rounded-md transition-all border"
                                    :class="
                                        isClassActive(
                                            'RemoteEvent',
                                            activeClasses,
                                        )
                                            ? 'bg-yellow-500/10 text-yellow-400 border-yellow-500/30'
                                            : 'text-muted-foreground hover:text-foreground hover:bg-card border-border/50'
                                    "
                                >
                                    <Zap class="w-3.5 h-3.5" />
                                </button>
                            </TooltipTrigger>
                            <TooltipContent side="bottom">
                                <p class="text-xs">RemoteEvent</p>
                            </TooltipContent>
                        </Tooltip>

                        <Tooltip>
                            <TooltipTrigger as-child>
                                <button
                                    @click="
                                        $emit('toggle-class', 'RemoteFunction')
                                    "
                                    class="relative flex items-center justify-center w-7 h-7 rounded-md transition-all border"
                                    :class="
                                        isClassActive(
                                            'RemoteFunction',
                                            activeClasses,
                                        )
                                            ? 'bg-purple-500/10 text-purple-400 border-purple-500/30'
                                            : 'text-muted-foreground hover:text-foreground hover:bg-card border-border/50'
                                    "
                                >
                                    <FunctionSquare class="w-3.5 h-3.5" />
                                </button>
                            </TooltipTrigger>
                            <TooltipContent side="bottom">
                                <p class="text-xs">RemoteFunction</p>
                            </TooltipContent>
                        </Tooltip>

                        <Tooltip>
                            <TooltipTrigger as-child>
                                <button
                                    @click="
                                        $emit(
                                            'toggle-class',
                                            'UnreliableRemoteEvent',
                                        )
                                    "
                                    class="relative flex items-center justify-center w-7 h-7 rounded-md transition-all border"
                                    :class="
                                        isClassActive(
                                            'UnreliableRemoteEvent',
                                            activeClasses,
                                        )
                                            ? 'bg-orange-500/10 text-orange-400 border-orange-500/30'
                                            : 'text-muted-foreground hover:text-foreground hover:bg-card border-border/50'
                                    "
                                >
                                    <Radio class="w-3.5 h-3.5" />
                                </button>
                            </TooltipTrigger>
                            <TooltipContent side="bottom">
                                <p class="text-xs">UnreliableRemoteEvent</p>
                            </TooltipContent>
                        </Tooltip>
                    </div>
                </div>
            </div>
        </div>
    </TooltipProvider>
</template>
