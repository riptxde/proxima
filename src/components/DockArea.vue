<script setup lang="ts">
import { Dock, DockIcon } from "@/components/ui/dock";
import { Play, Eraser, FolderOpen, Save } from "lucide-vue-next";
import DockLiquidGlass from "@/components/DockLiquidGlass.vue";
</script>

<template>
    <div class="flex justify-center items-center py-4">
        <DockLiquidGlass>
            <Dock class="!m-0">
                <DockIcon>
                    <Play class="size-5" />
                </DockIcon>
                <DockIcon>
                    <Eraser class="size-5" />
                </DockIcon>
                <DockIcon>
                    <FolderOpen class="size-5" />
                </DockIcon>
                <DockIcon>
                    <Save class="size-5" />
                </DockIcon>
            </Dock>
        </DockLiquidGlass>
    </div>
</template>
