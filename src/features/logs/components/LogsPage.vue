<script setup lang="ts">
import StarsBackground from "@/components/ui/bg-stars/StarsBackground.vue";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
    Search,
    Trash2,
    Info,
    AlertTriangle,
    XCircle,
    CheckCircle,
} from "lucide-vue-next";
import { useLogs } from "../composables/useLogs";
import type { LogLevel } from "../types/log";

const {
    logs,
    filters,
    clearLogs,
    toggleLevelFilter,
    setSearchFilter,
    getLevelCount,
} = useLogs();

const levelConfig = {
    info: {
        icon: Info,
        color: "text-blue-400",
        bgColor: "bg-blue-400/10",
        borderColor: "border-blue-400/20",
    },
    warning: {
        icon: AlertTriangle,
        color: "text-yellow-400",
        bgColor: "bg-yellow-400/10",
        borderColor: "border-yellow-400/20",
    },
    error: {
        icon: XCircle,
        color: "text-red-400",
        bgColor: "bg-red-400/10",
        borderColor: "border-red-400/20",
    },
    success: {
        icon: CheckCircle,
        color: "text-green-400",
        bgColor: "bg-green-400/10",
        borderColor: "border-green-400/20",
    },
};

const formatTimestamp = (date: Date) => {
    return date.toLocaleTimeString("en-US", {
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
        hour12: false,
    });
};

const isLevelActive = (level: LogLevel) => {
    return filters.value.levels.includes(level);
};
</script>

<template>
    <div
        class="h-full overflow-hidden flex flex-col relative bg-card rounded-lg border border-border shadow-sm"
    >
        <!-- Stars Background -->
        <div class="absolute inset-0 z-0 pointer-events-none rounded-lg">
            <StarsBackground :factor="0.05" :speed="50" star-color="#fff" />
        </div>

        <!-- Content -->
        <div
            class="relative z-10 flex-1 overflow-hidden p-4 flex flex-col gap-4"
        >
            <!-- Header -->
            <Card class="p-3">
                <div class="flex items-center justify-between">
                    <div
                        class="flex items-center gap-4 px-3 py-2 rounded-md border border-input bg-tab-bar"
                    >
                        <div
                            v-for="level in [
                                'info',
                                'success',
                                'warning',
                                'error',
                            ] as LogLevel[]"
                            :key="level"
                            class="flex items-center gap-2"
                        >
                            <component
                                :is="levelConfig[level].icon"
                                class="w-4 h-4 cursor-pointer transition-colors"
                                :class="
                                    isLevelActive(level)
                                        ? levelConfig[level].color
                                        : 'text-muted-foreground/40'
                                "
                                @click="toggleLevelFilter(level)"
                            />
                            <Badge
                                variant="outline"
                                class="text-xs bg-tab-bar select-none"
                            >
                                {{ getLevelCount(level) }}
                            </Badge>
                        </div>
                    </div>
                    <div class="flex items-center gap-2">
                        <div class="relative w-64">
                            <Search
                                class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground"
                            />
                            <Input
                                :model-value="filters.search"
                                @update:model-value="
                                    (value) => setSearchFilter(String(value))
                                "
                                placeholder="Search"
                                class="pl-9 h-9"
                            />
                        </div>
                        <Button
                            variant="outline"
                            size="icon"
                            @click="clearLogs"
                        >
                            <Trash2 class="w-4 h-4" />
                        </Button>
                    </div>
                </div>
            </Card>

            <!-- Terminal Output -->
            <Card class="flex-1 overflow-auto flex flex-col min-h-0 p-3">
                <div
                    v-if="logs.length === 0"
                    class="text-muted-foreground text-center font-mono text-sm select-none flex-1 flex items-center justify-center"
                >
                    No logs to display
                </div>
                <div v-else class="space-y-0.5 font-mono text-sm">
                    <div
                        v-for="log in logs"
                        :key="log.id"
                        class="flex items-start gap-3 py-2 px-3 rounded hover:bg-muted/50 transition-colors group border-l-2"
                        :class="[
                            levelConfig[log.level].borderColor,
                            levelConfig[log.level].bgColor,
                        ]"
                    >
                        <!-- Timestamp -->
                        <span
                            class="text-muted-foreground text-xs shrink-0 pt-0.5 mt-px select-none"
                        >
                            {{ formatTimestamp(log.timestamp) }}
                        </span>

                        <!-- Level Indicator -->
                        <div class="flex items-center gap-2 mt-[3px] shrink-0">
                            <component
                                :is="levelConfig[log.level].icon"
                                class="w-4 h-4 shrink-0"
                                :class="levelConfig[log.level].color"
                            />
                            <span
                                class="text-xs font-semibold uppercase tracking-wider w-16 select-none"
                                :class="levelConfig[log.level].color"
                            >
                                {{ log.level }}
                            </span>
                        </div>

                        <!-- Message -->
                        <span class="text-foreground flex-1 leading-relaxed">
                            {{ log.message }}
                        </span>
                    </div>
                </div>
            </Card>
        </div>
    </div>
</template>
