<script setup lang="ts">
import { ref, computed } from "vue";
import TitleBar from "./TitleBar.vue";
import Sidebar from "./Sidebar/Sidebar.vue";
import StarsBackground from "@/components/ui/bg-stars/StarsBackground.vue";
import LauncherStatusBar from "@/features/launcher/components/LauncherStatusBar.vue";
import { useLauncherProgress } from "@/features/launcher/composables/useLauncherProgress";

const parallaxStyle = ref({
    "--mouse-x": "0px",
    "--mouse-y": "0px",
});

const { isLaunching, queueCount } = useLauncherProgress();

// Show status bar when launching or when there are items in queue
const showStatusBar = computed(() => isLaunching.value || queueCount.value > 0);

function handleMouseMove(e: MouseEvent) {
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    const centerX = rect.width / 2;
    const centerY = rect.height / 2;

    const parallaxFactor = 0.1;

    // Calculate offset from center (inverted for opposite direction)
    const offsetX = -(e.clientX - rect.left - centerX) * parallaxFactor;
    const offsetY = -(e.clientY - rect.top - centerY) * parallaxFactor;

    parallaxStyle.value = {
        "--mouse-x": `${offsetX}px`,
        "--mouse-y": `${offsetY}px`,
    };
}
</script>

<template>
    <div class="h-screen w-screen flex flex-col bg-background">
        <!-- Top Bar -->
        <TitleBar />

        <!-- Launcher Status Bar (conditionally shown) -->
        <Transition
            enter-active-class="transition-all duration-300 ease-out"
            leave-active-class="transition-all duration-300 ease-in"
            enter-from-class="max-h-0 opacity-0"
            enter-to-class="max-h-20 opacity-100"
            leave-from-class="max-h-20 opacity-100"
            leave-to-class="max-h-0 opacity-0"
        >
            <div v-if="showStatusBar" class="overflow-hidden">
                <LauncherStatusBar />
            </div>
        </Transition>

        <!-- Main Content Area -->
        <div class="flex-1 overflow-hidden flex">
            <!-- Sidebar -->
            <Sidebar />

            <!-- Page Content -->
            <div
                class="flex-1 overflow-hidden bg-app-shell pr-2 pt-2 pb-2 relative -ml-2 -mt-2"
                @mousemove="handleMouseMove"
            >
                <!-- Rounded container for both stars and content -->
                <div class="absolute inset-2 rounded-lg overflow-hidden">
                    <!-- Stars Background -->
                    <StarsBackground
                        class="absolute inset-0 pointer-events-none"
                        :style="parallaxStyle"
                        star-color="#fff"
                    />

                    <!-- Page slot -->
                    <div class="relative z-10 h-full">
                        <slot />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
