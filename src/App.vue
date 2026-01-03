<script setup lang="ts">
import "vue-sonner/style.css";
import { computed, onMounted } from "vue";
import AppShell from "@/components/layout/AppShell.vue";
import EditorPage from "@/features/editor/components/EditorPage.vue";
import ScriptHubPage from "@/features/script-hub/components/ScriptHubPage.vue";
import ExplorerPage from "@/features/explorer/components/ExplorerPage.vue";
import RemoteSpyPage from "@/features/remote-spy/components/RemoteSpyPage.vue";
import LogsPage from "@/features/logs/components/LogsPage.vue";
import LauncherPage from "@/features/launcher/components/LauncherPage.vue";
import SettingsPage from "@/features/settings/components/SettingsPage.vue";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useNavigation } from "@/composables/useNavigation";
import { useExecutorClients } from "@/features/editor/composables/useExecutorClients";
import { useLogs } from "@/features/logs/composables/useLogs";
import { useHttpExecutor } from "@/features/editor/composables/useHttpExecutor";
import { useExplorer } from "@/features/explorer/composables/useExplorer";
import { useRemoteSpy } from "@/features/remote-spy/composables/useRemoteSpy";
import { useLauncherProgress } from "@/features/launcher/composables/useLauncherProgress";
import StartupAnimation from "@/components/shared/StartupAnimation.vue";

const { activePage } = useNavigation();
const { init: initExecutorClients } = useExecutorClients();
const { init: initLogs } = useLogs();
const { init: initHttpExecutor } = useHttpExecutor();
const { init: initExplorer } = useExplorer();
const { init: initRemoteSpy } = useRemoteSpy();
const { init: initLauncherProgress } = useLauncherProgress();

onMounted(() => {
    initExecutorClients();
    initLogs();
    initHttpExecutor();
    initExplorer();
    initRemoteSpy();
    initLauncherProgress();
});

const currentPageComponent = computed(() => {
    switch (activePage.value) {
        case "editor":
            return EditorPage;
        case "script-hub":
            return ScriptHubPage;
        case "explorer":
            return ExplorerPage;
        case "remote-spy":
            return RemoteSpyPage;
        case "logs":
            return LogsPage;
        case "launcher":
            return LauncherPage;
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
