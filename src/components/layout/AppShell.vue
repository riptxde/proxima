<script setup lang="ts">
import TitleBar from "./TitleBar.vue";
import Sidebar from "./Sidebar/Sidebar.vue";
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
            <div class="flex-1 overflow-hidden bg-app-shell pr-2 pb-2">
                <slot />
            </div>
        </div>
    </div>
</template>
