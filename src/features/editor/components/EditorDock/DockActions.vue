<script setup lang="ts">
import { computed } from "vue";
import { Dock, DockIcon } from "@/components/ui/dock";
import { Play, Eraser, FolderOpen, Save, Users } from "lucide-vue-next";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import { Kbd } from "@/components/ui/kbd";

const props = defineProps<{
    selectedCount: number;
}>();

defineEmits<{
    execute: [];
    clear: [];
    open: [];
    save: [];
    clients: [];
}>();

const hasSelectedClients = computed(() => props.selectedCount > 0);
</script>

<template>
    <Dock class="m-0!">
        <Tooltip>
            <TooltipTrigger as-child>
                <DockIcon
                    @click="$emit('execute')"
                    :class="!hasSelectedClients && 'cursor-not-allowed'"
                >
                    <Play
                        :class="[
                            'size-5 text-app-shell-foreground transition-opacity',
                            hasSelectedClients
                                ? 'opacity-60 group-hover:opacity-100'
                                : 'opacity-15',
                        ]"
                    />
                </DockIcon>
            </TooltipTrigger>
            <TooltipContent :side-offset="-15">
                <div class="flex items-center gap-2">
                    <p>Execute</p>
                    <Kbd>Ctrl+R</Kbd>
                </div>
            </TooltipContent>
        </Tooltip>

        <Tooltip>
            <TooltipTrigger as-child>
                <DockIcon @click="$emit('clients')">
                    <Users
                        class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                    />
                </DockIcon>
            </TooltipTrigger>
            <TooltipContent :side-offset="-15">
                <div class="flex items-center gap-2">
                    <p>Clients</p>
                    <Kbd>Alt+C</Kbd>
                </div>
            </TooltipContent>
        </Tooltip>

        <Tooltip>
            <TooltipTrigger as-child>
                <DockIcon @click="$emit('clear')">
                    <Eraser
                        class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                    />
                </DockIcon>
            </TooltipTrigger>
            <TooltipContent :side-offset="-15">
                <div class="flex items-center gap-2">
                    <p>Clear</p>
                    <Kbd>Alt+X</Kbd>
                </div>
            </TooltipContent>
        </Tooltip>

        <Tooltip>
            <TooltipTrigger as-child>
                <DockIcon @click="$emit('open')">
                    <FolderOpen
                        class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                    />
                </DockIcon>
            </TooltipTrigger>
            <TooltipContent :side-offset="-15">
                <div class="flex items-center gap-2">
                    <p>Open Script</p>
                    <Kbd>Ctrl+O</Kbd>
                </div>
            </TooltipContent>
        </Tooltip>

        <Tooltip>
            <TooltipTrigger as-child>
                <DockIcon @click="$emit('save')">
                    <Save
                        class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                    />
                </DockIcon>
            </TooltipTrigger>
            <TooltipContent :side-offset="-15">
                <div class="flex items-center gap-2">
                    <p>Save</p>
                    <Kbd>Ctrl+S</Kbd>
                </div>
            </TooltipContent>
        </Tooltip>
    </Dock>
</template>
