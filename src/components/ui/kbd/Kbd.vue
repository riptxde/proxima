<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { cn } from "@/lib/utils";

const props = defineProps<{
    class?: HTMLAttributes["class"];
}>();
</script>

<template>
    <kbd
        :class="
            cn(
                'bg-muted text-muted-foreground pointer-events-none z-100 inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1 rounded-sm px-1 font-sans text-xs font-medium select-none',
                '[&_svg:not([class*=\'size-\'])]:size-3',
                'in-data-[slot=tooltip-content]:bg-[#262626] in-data-[slot=tooltip-content]:text-foreground',
                props.class,
            )
        "
    >
        <slot />
    </kbd>
</template>
