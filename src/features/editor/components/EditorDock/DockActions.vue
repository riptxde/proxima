<script setup lang="ts">
import { computed } from "vue";
import { Dock, DockIcon } from "@/components/ui/dock";
import { Play, Eraser, FolderOpen, Save, Users } from "lucide-vue-next";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@/components/ui/tooltip";

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
                <p>Execute</p>
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
                <p>Clients</p>
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
                <p>Clear</p>
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
                <p>Open Script</p>
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
                <p>Save</p>
            </TooltipContent>
        </Tooltip>
    </Dock>
</template>
