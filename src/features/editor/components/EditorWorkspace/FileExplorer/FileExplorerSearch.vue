<script setup lang="ts">
import { Input } from "@/components/ui/input";
import { Search } from "lucide-vue-next";

interface Props {
    modelValue: string;
}

defineProps<Props>();

const emit = defineEmits<{
    "update:modelValue": [value: string];
}>();
</script>

<template>
    <div class="relative">
        <Search
            class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground"
        />
        <Input
            :value="modelValue"
            @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
            type="text"
            placeholder="Search"
            class="pl-9 border-none h-[46px] text-sm bg-tab-bar"
        />
    </div>
</template>
