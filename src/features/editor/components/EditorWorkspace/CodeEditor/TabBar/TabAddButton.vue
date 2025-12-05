<script setup lang="ts">
import { Plus } from "lucide-vue-next";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import { Kbd } from "@/components/ui/kbd";

defineEmits<{
    click: [];
}>();
</script>

<template>
    <Tooltip>
        <TooltipTrigger as-child>
            <button
                @click="$emit('click')"
                class="flex items-center justify-center w-7 h-7 rounded text-muted-foreground hover:text-foreground hover:bg-accent transition-all"
            >
                <Plus class="h-4 w-4" />
            </button>
        </TooltipTrigger>
        <TooltipContent>
            <div class="flex items-center gap-2">
                <p>New Tab</p>
                <Kbd>Ctrl+T</Kbd>
            </div>
        </TooltipContent>
    </Tooltip>
</template>
