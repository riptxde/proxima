<script setup lang="ts">
import { useLauncherProgress } from "@/features/launcher/composables/useLauncherProgress";
import { Progress } from "@/components/ui/progress";

const { isLaunching, queueCount, launchProgress, launchStatus, launchError } =
    useLauncherProgress();
</script>

<template>
    <div class="px-4 py-3 space-y-2 bg-card">
        <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
                <div
                    class="w-2 h-2 rounded-full transition-all"
                    :class="{
                        'bg-green-500 animate-pulse':
                            isLaunching && !launchError,
                        'bg-red-500': launchError,
                        'bg-muted-foreground': !isLaunching && !launchError,
                    }"
                ></div>
                <p
                    class="text-sm font-medium transition-colors"
                    :class="{
                        'text-destructive': launchError,
                    }"
                >
                    {{ launchError || launchStatus }}
                </p>
            </div>
            <div class="flex items-center gap-3">
                <span
                    v-if="queueCount > 0"
                    class="text-xs text-muted-foreground"
                >
                    {{ queueCount }} in queue
                </span>
                <span
                    v-if="isLaunching && !launchError"
                    class="text-xs text-muted-foreground font-mono"
                >
                    {{ launchProgress }}%
                </span>
            </div>
        </div>
        <Progress
            :model-value="launchProgress"
            class="h-1"
            :class="{
                'opacity-30 animate-pulse': !isLaunching && !launchError,
                'opacity-100': isLaunching || launchError,
            }"
        />
    </div>
</template>
