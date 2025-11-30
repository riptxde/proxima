<script setup lang="ts">
import { ref } from "vue";
import { Dock, DockIcon } from "@/components/ui/dock";
import { Play, Eraser, FolderOpen, Save } from "lucide-vue-next";
import DockLiquidGlass from "@/components/DockLiquidGlass.vue";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import { useTabState } from "@/composables/useTabState";

const { clearActiveTab, openFileAsTab } = useTabState();

const fileInputRef = ref<HTMLInputElement | null>(null);

const handleOpenScript = () => {
    fileInputRef.value?.click();
};

const handleFileChange = async (event: Event) => {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];

    if (!file) return;

    const content = await file.text();
    openFileAsTab(file.name, content);

    // Reset input so the same file can be opened again
    input.value = "";
};
</script>

<template>
    <div class="flex justify-center items-center py-4">
        <!-- Hidden file input -->
        <input
            ref="fileInputRef"
            type="file"
            accept=".lua,.luau,.txt"
            @change="handleFileChange"
            class="hidden"
        />

        <TooltipProvider>
            <DockLiquidGlass>
                <Dock class="m-0!">
                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon>
                                <Play
                                    class="size-5 text-sidebar-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Execute</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon @click="clearActiveTab">
                                <Eraser
                                    class="size-5 text-sidebar-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Clear</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon @click="handleOpenScript">
                                <FolderOpen
                                    class="size-5 text-sidebar-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Open Script</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon>
                                <Save
                                    class="size-5 text-sidebar-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Save</p>
                        </TooltipContent>
                    </Tooltip>
                </Dock>
            </DockLiquidGlass>
        </TooltipProvider>
    </div>
</template>
