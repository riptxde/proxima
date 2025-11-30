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
        <div class="w-16 bg-app-shell flex flex-col items-center py-1 gap-2">
            <Tooltip>
                <TooltipTrigger as-child>
                    <button
                        class="h-12 w-12 flex items-center justify-center rounded-lg bg-app-shell-accent text-app-shell-accent-foreground"
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
                        class="h-12 w-12 flex items-center justify-center rounded-lg text-app-shell-foreground opacity-60 hover:opacity-100 transition-opacity"
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
                        class="h-12 w-12 flex items-center justify-center rounded-lg text-app-shell-foreground opacity-60 hover:opacity-100 transition-opacity"
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
                        class="h-12 w-12 flex items-center justify-center rounded-lg text-app-shell-foreground opacity-60 hover:opacity-100 transition-opacity"
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
