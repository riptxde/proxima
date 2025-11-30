<script setup lang="ts">
import "vue-sonner/style.css";
import { computed } from "vue";
import AppShell from "@/components/layout/AppShell.vue";
import EditorPage from "@/features/editor/components/EditorPage.vue";
import ScriptHubPage from "@/features/script-hub/components/ScriptHubPage.vue";
import LogsPage from "@/features/logs/components/LogsPage.vue";
import SettingsPage from "@/features/settings/components/SettingsPage.vue";
import { Toaster } from "@/components/ui/sonner";
import { useNavigation } from "@/composables/useNavigation";

const { activePage } = useNavigation();

const currentPageComponent = computed(() => {
    switch (activePage.value) {
        case "editor":
            return EditorPage;
        case "script-hub":
            return ScriptHubPage;
        case "logs":
            return LogsPage;
        case "settings":
            return SettingsPage;
        default:
            return EditorPage;
    }
});
</script>

<template>
    <AppShell>
        <component :is="currentPageComponent" />
    </AppShell>
    <Toaster position="top-center" :duration="2000" />
</template>
