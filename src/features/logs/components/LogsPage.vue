<script setup lang="ts">
import StarsBackground from "@/components/ui/bg-stars/StarsBackground.vue";
import { Card } from "@/components/ui/card";
</script>

<template>
    <div
        class="h-full overflow-hidden flex flex-col relative bg-card rounded-lg border border-border shadow-sm"
    >
        <!-- Stars Background -->
        <div class="absolute inset-0 z-0 pointer-events-none rounded-lg">
            <StarsBackground :factor="0.05" :speed="50" star-color="#fff" />
        </div>

        <Card class="relative z-10 h-full m-4 flex items-center justify-center">
            <p class="text-muted-foreground text-lg">Logs - Coming Soon</p>
        </Card>
    </div>
</template>
