<script setup lang="ts">
import { Checkbox } from "@/components/ui/checkbox";
import type { Client } from "@/types/client";

defineProps<{
    client: Client;
    selected: boolean;
}>();

defineEmits<{
    toggle: [id: string];
}>();
</script>

<template>
    <div
        class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-muted/50 transition-colors"
    >
        <Checkbox
            :model-value="selected"
            @update:model-value="$emit('toggle', client.id)"
        />
        <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-foreground">
                {{ client.username }}
            </div>
            <div class="text-xs text-muted-foreground truncate">
                ID: {{ client.id }}
            </div>
        </div>
    </div>
</template>
