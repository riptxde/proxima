<script setup lang="ts">
import "vue-sonner/style.css";
import { computed, onMounted } from "vue";
import AppShell from "@/components/layout/AppShell.vue";
import EditorPage from "@/features/editor/components/EditorPage.vue";
import ScriptHubPage from "@/features/script-hub/components/ScriptHubPage.vue";
import ExplorerPage from "@/features/explorer/components/ExplorerPage.vue";
import LogsPage from "@/features/logs/components/LogsPage.vue";
import SettingsPage from "@/features/settings/components/SettingsPage.vue";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useNavigation } from "@/composables/useNavigation";
import { useClients } from "@/features/editor/composables/useClients";
import { useLogs } from "@/features/logs/composables/useLogs";
import { useHttpExecutor } from "@/features/editor/composables/useHttpExecutor";
import { useExplorer } from "@/features/explorer/composables/useExplorer";
import StartupAnimation from "@/components/shared/StartupAnimation.vue";

const { activePage } = useNavigation();
const { initialize } = useClients();
const { initializeLogListener } = useLogs();
const { initialize: initHttpExecutor } = useHttpExecutor();
const { initializeExplorerClientListeners } = useExplorer();

onMounted(() => {
  initialize();
  initializeLogListener();
  initHttpExecutor();
  initializeExplorerClientListeners();
});

const currentPageComponent = computed(() => {
  switch (activePage.value) {
    case "editor":
      return EditorPage;
    case "script-hub":
      return ScriptHubPage;
    case "explorer":
      return ExplorerPage;
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
  <StartupAnimation />
  <TooltipProvider>
    <AppShell>
      <component :is="currentPageComponent" />
    </AppShell>
    <Toaster position="top-center" :duration="2000" />
  </TooltipProvider>
</template>
