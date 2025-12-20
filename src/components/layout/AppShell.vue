<script setup lang="ts">
import { ref } from "vue";
import TitleBar from "./TitleBar.vue";
import Sidebar from "./Sidebar/Sidebar.vue";
import StarsBackground from "@/components/ui/bg-stars/StarsBackground.vue";

const parallaxStyle = ref({
  "--mouse-x": "0px",
  "--mouse-y": "0px",
});

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

    <!-- Main Content Area -->
    <div class="flex-1 overflow-hidden flex">
      <!-- Sidebar -->
      <Sidebar />

      <!-- Page Content -->
      <div
        class="flex-1 overflow-hidden bg-app-shell pr-2 pt-2 pb-2 relative -ml-2"
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
