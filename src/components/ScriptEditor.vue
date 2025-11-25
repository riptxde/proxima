<script setup lang="ts">
import { ref } from "vue";
import { Textarea } from "@/components/ui/textarea";

const scriptContent = ref("");
</script>

<template>
    <div class="h-full overflow-hidden">
        <Textarea
            v-model="scriptContent"
            placeholder="-- Write your script here..."
            class="h-full w-full resize-none font-mono text-sm rounded-none border-0 px-4 py-3 shadow-none focus-visible:ring-0"
        />
    </div>
</template>
