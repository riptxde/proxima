<script setup lang="ts">
import TitleBar from "./TitleBar.vue";
import Sidebar from "./Sidebar/Sidebar.vue";
import StarsBackground from "@/components/ui/bg-stars/StarsBackground.vue";
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
      <div class="flex-1 overflow-hidden bg-app-shell p-2 relative">
        <!-- Stars Background (inside the inset with rounded corners) -->
        <div
          class="absolute inset-2 pointer-events-none rounded-lg overflow-hidden"
        >
          <StarsBackground :factor="0.05" :speed="50" star-color="#fff" />
        </div>

        <!-- Page slot (inside the inset with rounded corners) -->
        <div class="relative z-10 h-full rounded-lg overflow-hidden">
          <slot />
        </div>
      </div>
    </div>
  </div>
</template>
