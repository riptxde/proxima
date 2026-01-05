<script setup lang="ts">
import {
    CodeXml,
    Library,
    Settings,
    Logs,
    ScanSearch,
    Zap,
    Rocket,
} from "lucide-vue-next";
import { computed } from "vue";
import SidebarButton from "./SidebarButton.vue";
import { useNavigation } from "@/composables/useNavigation";
import type { SidebarButton as SidebarButtonType } from "./types";

const { navigate, isActive, activePage } = useNavigation();

const buttons: SidebarButtonType[] = [
    { id: "editor", label: "Editor", icon: CodeXml },
    { id: "logs", label: "Logs", icon: Logs },
    { id: "script-hub", label: "Script Hub", icon: Library },
    { id: "explorer", label: "Explorer", icon: ScanSearch },
    { id: "remote-spy", label: "Remote Spy", icon: Zap },
    { id: "launcher", label: "Roblox Launcher", icon: Rocket },
    { id: "settings", label: "Settings", icon: Settings },
];

const activeButtonIndex = computed(() => {
    return buttons.findIndex((button) => button.id === activePage.value);
});

const indicatorStyle = computed(() => {
    const index = activeButtonIndex.value;
    if (index === -1) return { transform: "translateY(0px)" };

    // Calculate position: py-1 (4px) + (index * (h-12 (48px) + gap-2 (8px)))
    const topPadding = 4;
    const buttonHeight = 48;
    const gap = 8;
    const offset = topPadding + index * (buttonHeight + gap);

    return {
        transform: `translateY(${offset}px)`,
    };
});
</script>

<template>
    <div
        class="w-16 bg-app-shell flex flex-col items-center py-1 gap-2 relative"
    >
        <!-- Animated indicator -->
        <div
            class="absolute top-0 left-1/2 -translate-x-1/2 w-12 h-12 rounded-lg bg-app-shell-accent transition-transform duration-300 ease-out pointer-events-none"
            :style="indicatorStyle"
        />

        <SidebarButton
            v-for="button in buttons"
            :key="button.id"
            :icon="button.icon"
            :label="button.label"
            :is-active="isActive(button.id)"
            @click="navigate(button.id)"
        />
    </div>
</template>
