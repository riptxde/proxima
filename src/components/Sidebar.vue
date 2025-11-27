<script setup lang="ts">
import { CodeXml, Library, Settings, Logs } from "lucide-vue-next";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip";
</script>

<template>
    <TooltipProvider>
        <div class="w-16 bg-sidebar flex flex-col items-center py-1 gap-2">
            <Tooltip>
                <TooltipTrigger as-child>
                    <button
                        class="h-12 w-12 flex items-center justify-center rounded-lg bg-sidebar-accent text-sidebar-accent-foreground transition-colors"
                    >
                        <CodeXml :size="20" />
                    </button>
                </TooltipTrigger>
                <TooltipContent side="right">
                    <p>Editor</p>
                </TooltipContent>
            </Tooltip>

            <Tooltip>
                <TooltipTrigger as-child>
                    <button
                        class="h-12 w-12 flex items-center justify-center rounded-lg hover:bg-sidebar-accent hover:text-sidebar-accent-foreground text-sidebar-foreground/60 transition-colors"
                    >
                        <Library :size="20" />
                    </button>
                </TooltipTrigger>
                <TooltipContent side="right">
                    <p>Script Hub</p>
                </TooltipContent>
            </Tooltip>

            <Tooltip>
                <TooltipTrigger as-child>
                    <button
                        class="h-12 w-12 flex items-center justify-center rounded-lg hover:bg-sidebar-accent hover:text-sidebar-accent-foreground text-sidebar-foreground/60 transition-colors"
                    >
                        <Logs :size="20" />
                    </button>
                </TooltipTrigger>
                <TooltipContent side="right">
                    <p>Logs</p>
                </TooltipContent>
            </Tooltip>

            <Tooltip>
                <TooltipTrigger as-child>
                    <button
                        class="h-12 w-12 flex items-center justify-center rounded-lg hover:bg-sidebar-accent hover:text-sidebar-accent-foreground text-sidebar-foreground/60 transition-colors"
                    >
                        <Settings :size="20" />
                    </button>
                </TooltipTrigger>
                <TooltipContent side="right">
                    <p>Settings</p>
                </TooltipContent>
            </Tooltip>
        </div>
    </TooltipProvider>
</template>
