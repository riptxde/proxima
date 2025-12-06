<script setup lang="ts">
import { Card } from "@/components/ui/card";
import { Info, AlertTriangle, XCircle, CheckCircle } from "lucide-vue-next";
import { useLogs } from "@/features/logs/composables/useLogs";
import type { LogLevel } from "@/features/logs/types/log";

const { allLogs } = useLogs();

const levelConfig: Record<LogLevel, { icon: any; color: string }> = {
    info: {
        icon: Info,
        color: "text-blue-500",
    },
    success: {
        icon: CheckCircle,
        color: "text-green-500",
    },
    warning: {
        icon: AlertTriangle,
        color: "text-yellow-500",
    },
    error: {
        icon: XCircle,
        color: "text-red-500",
    },
};

const formatTime = (date: Date) => {
    return date.toLocaleTimeString();
};
</script>

<template>
    <Card class="h-full overflow-y-auto p-2 rounded-t-none border-t-0">
        <div
            v-if="allLogs.length === 0"
            class="text-muted-foreground text-center font-mono text-sm select-none h-full flex items-center justify-center"
        >
            No logs to display
        </div>
        <div v-else class="space-y-1 font-mono text-xs">
            <div
                v-for="log in allLogs"
                :key="log.id"
                class="flex items-start gap-2 px-2 py-1.5 hover:bg-muted/50 rounded"
            >
                <!-- Timestamp -->
                <span class="text-muted-foreground whitespace-nowrap shrink-0">
                    {{ formatTime(log.timestamp) }}
                </span>

                <!-- Level Icon -->
                <component
                    :is="levelConfig[log.level].icon"
                    :size="16"
                    class="shrink-0"
                    style="min-width: 16px; min-height: 16px"
                    :class="levelConfig[log.level].color"
                />

                <!-- Message -->
                <span
                    class="text-foreground flex-1 min-w-0 wrap-break-word whitespace-pre-wrap"
                >
                    {{ log.message }}
                </span>
            </div>
        </div>
    </Card>
</template>
