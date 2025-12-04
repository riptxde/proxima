<script setup lang="ts">
import { watch } from "vue";
import { X, Minus, Square } from "lucide-vue-next";
import Icon from "@/assets/icon.svg";
import { getCurrentWindow } from "@tauri-apps/api/window";
import RadiantText from "@/components/ui/radiant-text/RadiantText.vue";
import { useSettings } from "@/features/settings/composables/useSettings";
import { toast } from "vue-sonner";

const appWindow = getCurrentWindow();
const { applicationSettings } = useSettings();

const minimizeWindow = async () => {
    await appWindow.minimize();
};

const toggleMaximize = async () => {
    await appWindow.toggleMaximize();
};

const closeWindow = async () => {
    await appWindow.close();
};

const showCredits = () => {
    toast("Credits", {
        description: "Developer: riptxde",
    });
};

// Watch for always on top setting changes
watch(
    () => applicationSettings.value.alwaysOnTop,
    async (alwaysOnTop) => {
        await appWindow.setAlwaysOnTop(alwaysOnTop);
    },
    { immediate: true },
);
</script>

<template>
    <div
        data-tauri-drag-region
        class="h-12 bg-app-shell flex items-center justify-between px-4 select-none"
    >
        <div class="flex items-center gap-2" data-tauri-drag-region>
            <img
                :src="Icon"
                alt="Proxima Icon"
                class="h-6 brightness-0 invert"
                data-tauri-drag-region
            />
            <RadiantText
                :duration="4"
                :radiant-width="100"
                class="font-title text-xl tracking-wider transition ease-out hover:text-white hover:duration-300 mt-1.5"
                @click="showCredits"
                style="
                    font-feature-settings:
                        &quot;liga&quot; 1,
                        &quot;calt&quot; 1;
                    -webkit-app-region: no-drag;
                "
            >
                PROXIMA
            </RadiantText>
        </div>
        <div class="flex items-center gap-2 relative z-10">
            <button
                @click="minimizeWindow"
                class="h-6 w-6 flex items-center justify-center hover:bg-muted rounded transition-colors cursor-pointer"
                type="button"
            >
                <Minus :size="16" />
            </button>
            <button
                @click="toggleMaximize"
                class="h-6 w-6 flex items-center justify-center hover:bg-muted rounded transition-colors cursor-pointer"
                type="button"
            >
                <Square :size="14" />
            </button>
            <button
                @click="closeWindow"
                class="h-6 w-6 flex items-center justify-center hover:bg-destructive hover:text-destructive-foreground rounded transition-colors cursor-pointer"
                type="button"
            >
                <X :size="16" />
            </button>
        </div>
    </div>
</template>

<style scoped>
/* Prevent drag region on buttons */
button {
    -webkit-app-region: no-drag;
}
</style>
