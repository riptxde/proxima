<script setup lang="ts">
import { Button } from "@/components/ui/button";
</script>

<template>
    <div class="border-t bg-muted/30 p-2 flex items-center gap-2">
        <Button variant="default" size="sm">Execute</Button>
        <Button variant="outline" size="sm">Clear</Button>
        <Button variant="outline" size="sm">Open</Button>
        <Button variant="outline" size="sm">Save</Button>
    </div>
</template>
