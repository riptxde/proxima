<script setup lang="ts">
import { Badge } from "@/components/ui/badge";
import {
    ArrowUp,
    ArrowDown,
    Zap,
    FunctionSquare,
    Radio,
} from "lucide-vue-next";
import type { Remote, RemoteCall, RemoteClass } from "../types/remote-spy";

interface Props {
    remote: Remote;
    call: RemoteCall;
}

defineProps<Props>();

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

const getClassIcon = (remoteClass: RemoteClass) => {
    if (remoteClass === "RemoteEvent") return Zap;
    if (remoteClass === "RemoteFunction") return FunctionSquare;
    return Radio; // UnreliableRemoteEvent
};

const getClassColor = (remoteClass: RemoteClass) => {
    if (remoteClass === "RemoteEvent") return "text-yellow-400";
    if (remoteClass === "RemoteFunction") return "text-purple-400";
    return "text-orange-400"; // UnreliableRemoteEvent
};
</script>

<template>
    <div class="p-5 space-y-4">
        <!-- Header Section -->
        <div class="space-y-3">
            <!-- Remote Name -->
            <div>
                <h2 class="text-xl font-bold tracking-tight">
                    {{ remote.name }}
                </h2>
            </div>

            <!-- Metadata Badges -->
            <div class="flex items-center gap-2 flex-wrap">
                <!-- Timestamp Badge -->
                <div
                    class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md bg-muted/60 border border-border/40"
                >
                    <span
                        class="text-[10px] text-muted-foreground/70 uppercase tracking-wider font-medium"
                        >Time</span
                    >
                    <span class="text-xs font-mono">{{
                        formatTime(call.timestamp)
                    }}</span>
                </div>

                <!-- Direction Badge -->
                <div
                    class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md border"
                    :class="
                        call.direction === 'outgoing'
                            ? 'bg-green-500/10 border-green-500/30'
                            : 'bg-blue-500/10 border-blue-500/30'
                    "
                >
                    <component
                        :is="getDirectionIcon(call.direction)"
                        class="w-3 h-3"
                        :class="getDirectionColor(call.direction)"
                    />
                    <span class="text-xs font-medium capitalize">{{
                        call.direction
                    }}</span>
                </div>

                <!-- Class Badge -->
                <div
                    class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md border"
                    :class="{
                        'bg-yellow-500/10 border-yellow-500/30':
                            remote.class === 'RemoteEvent',
                        'bg-purple-500/10 border-purple-500/30':
                            remote.class === 'RemoteFunction',
                        'bg-orange-500/10 border-orange-500/30':
                            remote.class === 'UnreliableRemoteEvent',
                    }"
                >
                    <component
                        :is="getClassIcon(remote.class)"
                        class="w-3 h-3"
                        :class="getClassColor(remote.class)"
                    />
                    <span class="text-xs font-medium">{{ remote.class }}</span>
                </div>
            </div>
        </div>

        <!-- Divider -->
        <div class="h-px bg-border/60"></div>

        <!-- Path Information -->
        <div class="space-y-3">
            <!-- Remote Path -->
            <div>
                <div
                    class="text-[10px] text-muted-foreground/70 uppercase tracking-wider font-semibold mb-2"
                >
                    Remote Path
                </div>
                <div
                    class="px-3 py-2.5 rounded-md border border-border/60 bg-muted/40"
                >
                    <div
                        class="text-xs font-mono break-all leading-relaxed text-foreground/90"
                    >
                        {{ remote.path }}
                    </div>
                </div>
            </div>

            <!-- Calling Script -->
            <div v-if="call.callingScriptName">
                <div
                    class="text-[10px] text-muted-foreground/70 uppercase tracking-wider font-semibold mb-2"
                >
                    Calling Script
                </div>
                <div
                    class="px-3 py-2.5 rounded-md border border-border/60 bg-muted/40"
                >
                    <div
                        class="text-xs font-mono break-all leading-relaxed text-foreground/90"
                    >
                        {{ call.callingScriptPath }}
                    </div>
                </div>
            </div>
        </div>

        <!-- Arguments Section -->
        <div v-if="call.arguments.length > 0" class="space-y-3">
            <div
                class="text-[10px] text-muted-foreground/70 uppercase tracking-wider font-semibold"
            >
                Arguments ({{ call.arguments.length }})
            </div>

            <div class="space-y-2.5">
                <div
                    v-for="(arg, index) in call.arguments"
                    :key="index"
                    class="group rounded-md border border-border/60 bg-muted/30 overflow-hidden transition-all hover:border-border hover:bg-muted/50"
                >
                    <!-- Argument Header -->
                    <div
                        class="flex items-center justify-between px-3 py-2 bg-muted/50 border-b border-border/40"
                    >
                        <div class="flex items-center gap-2">
                            <span
                                class="text-xs font-mono font-semibold text-sidebar-primary"
                                >{{ index + 1 }}</span
                            >
                            <div class="w-px h-3.5 bg-border/60"></div>
                            <Badge
                                variant="secondary"
                                class="text-[10px] font-mono px-1.5 py-0 h-5 bg-background/60"
                            >
                                {{ arg.type }}
                            </Badge>
                        </div>
                    </div>

                    <!-- Argument Value -->
                    <div class="px-3 py-2.5">
                        <div
                            class="text-xs font-mono break-all leading-relaxed text-foreground/90"
                        >
                            {{ arg.value }}
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Return Values Section -->
        <div
            v-if="call.returnValues && call.returnValues.length > 0"
            class="space-y-3"
        >
            <div
                class="text-[10px] text-muted-foreground/70 uppercase tracking-wider font-semibold"
            >
                {{
                    call.returnValues.length === 1
                        ? "Return Value"
                        : "Return Values"
                }}
            </div>

            <div
                v-for="(returnValue, index) in call.returnValues"
                :key="index"
                class="group rounded-md border border-green-500/30 bg-green-500/5 overflow-hidden transition-all hover:border-green-500/50 hover:bg-green-500/10"
            >
                <!-- Return Header -->
                <div
                    class="flex items-center justify-between px-3 py-2 bg-green-500/10 border-b border-green-500/20"
                >
                    <div class="flex items-center gap-2">
                        <span
                            class="text-xs font-mono font-semibold text-green-400"
                            >return</span
                        >
                        <Badge
                            v-if="call.returnValues.length > 1"
                            variant="outline"
                            class="text-[10px] font-mono px-1.5 py-0 h-5 bg-background/60 border-green-500/20"
                        >
                            {{ index + 1 }}
                        </Badge>
                        <div class="w-px h-3.5 bg-green-500/30"></div>
                        <Badge
                            variant="secondary"
                            class="text-[10px] font-mono px-1.5 py-0 h-5 bg-background/60 border-green-500/20"
                        >
                            {{ returnValue.type }}
                        </Badge>
                    </div>
                </div>

                <!-- Return Value -->
                <div class="px-3 py-2.5">
                    <div
                        class="text-xs font-mono break-all leading-relaxed text-foreground/90"
                    >
                        {{ returnValue.value }}
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
