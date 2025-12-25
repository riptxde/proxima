<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import Tab from "./Tab.vue";
import TabAddButton from "./TabAddButton.vue";
import type { Tab as TabType } from "@/features/editor/types/tab";

interface Props {
    tabs: TabType[];
    activeTabId: number;
}

defineProps<Props>();

const emit = defineEmits<{
    addTab: [];
    selectTab: [tabId: number];
    renameTab: [tabId: number, newName: string];
    closeTab: [tabId: number];
}>();

const tabBarRef = ref<HTMLElement | null>(null);
let targetScrollLeft = 0;
let currentScrollLeft = 0;
let animationFrameId: number | null = null;

const smoothScroll = () => {
    if (!tabBarRef.value) return;

    // Smooth interpolation towards target
    const diff = targetScrollLeft - currentScrollLeft;
    const delta = diff * 0.15; // Smoothing factor

    if (Math.abs(diff) < 0.5) {
        // Close enough, snap to target and stop
        currentScrollLeft = targetScrollLeft;
        tabBarRef.value.scrollLeft = targetScrollLeft;
        animationFrameId = null;
        return;
    }

    currentScrollLeft += delta;
    tabBarRef.value.scrollLeft = currentScrollLeft;

    animationFrameId = requestAnimationFrame(smoothScroll);
};

const handleWheel = (e: WheelEvent) => {
    if (!tabBarRef.value) return;

    // Prevent default vertical scroll
    e.preventDefault();

    // Update target scroll position
    targetScrollLeft += e.deltaY;

    // Clamp to valid range
    const maxScroll = tabBarRef.value.scrollWidth - tabBarRef.value.clientWidth;
    targetScrollLeft = Math.max(0, Math.min(targetScrollLeft, maxScroll));

    // Initialize current scroll if first time
    if (animationFrameId === null) {
        currentScrollLeft = tabBarRef.value.scrollLeft;
        animationFrameId = requestAnimationFrame(smoothScroll);
    }
};

onMounted(() => {
    tabBarRef.value?.addEventListener("wheel", handleWheel, { passive: false });
});

onUnmounted(() => {
    tabBarRef.value?.removeEventListener("wheel", handleWheel);
    if (animationFrameId !== null) {
        cancelAnimationFrame(animationFrameId);
    }
});
</script>

<template>
    <div
        ref="tabBarRef"
        class="flex items-center gap-1 bg-tab-bar rounded-md p-1.5 overflow-x-auto"
    >
        <Tab
            v-for="tab in tabs"
            :key="tab.id"
            :id="tab.id"
            :name="tab.name"
            :is-active="tab.id === activeTabId"
            :show-close="tabs.length > 1"
            :file-path="tab.filePath"
            @select="(tabId) => emit('selectTab', tabId)"
            @rename="(tabId, newName) => emit('renameTab', tabId, newName)"
            @close="(tabId) => emit('closeTab', tabId)"
        />

        <TabAddButton @click="emit('addTab')" />
    </div>
</template>
