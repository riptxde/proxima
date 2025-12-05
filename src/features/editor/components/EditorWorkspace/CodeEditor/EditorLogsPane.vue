<script setup lang="ts">
import { ref } from "vue";
import { Card } from "@/components/ui/card";
import { Info, AlertTriangle, XCircle, CheckCircle } from "lucide-vue-next";

interface DummyLog {
    id: string;
    timestamp: string;
    level: "info" | "success" | "warning" | "error";
    message: string;
}

const dummyLogs = ref<DummyLog[]>([
    {
        id: "1",
        timestamp: new Date().toLocaleTimeString(),
        level: "info",
        message: "Script editor initialized",
    },
    {
        id: "2",
        timestamp: new Date().toLocaleTimeString(),
        level: "success",
        message: "File loaded successfully: script.lua",
    },
    {
        id: "3",
        timestamp: new Date().toLocaleTimeString(),
        level: "warning",
        message: "No clients connected",
    },
    {
        id: "4",
        timestamp: new Date().toLocaleTimeString(),
        level: "error",
        message: "Failed to execute script: Invalid syntax",
    },
    {
        id: "5",
        timestamp: new Date().toLocaleTimeString(),
        level: "success",
        message: "Script executed on 2 clients",
    },
]);

const levelConfig = {
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
</script>

<template>
    <Card class="h-full overflow-y-auto p-2 rounded-t-none border-t-0">
        <div class="space-y-1 font-mono text-xs">
            <div
                v-for="log in dummyLogs"
                :key="log.id"
                class="flex items-start gap-2 px-2 py-1.5 hover:bg-muted/50 rounded"
            >
                <!-- Timestamp -->
                <span class="text-muted-foreground whitespace-nowrap">
                    {{ log.timestamp }}
                </span>

                <!-- Level Icon -->
                <component
                    :is="levelConfig[log.level].icon"
                    class="w-4 h-4"
                    :class="levelConfig[log.level].color"
                />

                <!-- Message -->
                <span class="text-foreground flex-1">
                    {{ log.message }}
                </span>
            </div>
        </div>
    </Card>
</template>
