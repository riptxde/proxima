<script setup lang="ts">
import { ref, onMounted } from "vue";
import logoSvg from "@/assets/icon.svg?raw";

const isVisible = ref(true);
const isAnimating = ref(false);

onMounted(() => {
    // Small delay before starting animation
    setTimeout(() => {
        isAnimating.value = true;
    }, 100);

    // Hide component after animation completes
    setTimeout(() => {
        isVisible.value = false;
    }, 1600);
});
</script>

<template>
    <Transition name="fade">
        <div
            v-if="isVisible"
            class="fixed inset-0 z-9999 flex items-center justify-center pointer-events-none"
        >
            <div
                class="logo-container"
                :class="{ animating: isAnimating }"
                v-html="logoSvg"
            />
        </div>
    </Transition>
</template>

<style scoped>
.logo-container {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    transform: scale(10);
    transition: all 1.5s cubic-bezier(0.68, -0.55, 0.265, 1.4);
}

.logo-container :deep(svg) {
    width: 100vw;
    height: 100vh;
    filter: invert(1);
    transition: inherit;
}

.logo-container.animating {
    transform: scale(0);
}

@keyframes fadeOut {
    0% {
        opacity: 1;
    }
    20% {
        opacity: 1;
    }
    100% {
        opacity: 0;
    }
}

.logo-container.animating {
    animation: fadeOut 1.5s cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

.fade-leave-active {
    transition: opacity 0.2s ease;
}

.fade-leave-to {
    opacity: 0;
}
</style>
