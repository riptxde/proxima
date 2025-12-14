<script setup lang="ts">
import {
  CodeXml,
  Library,
  Settings,
  Logs,
  PackageSearch,
} from "lucide-vue-next";
import SidebarButton from "./SidebarButton.vue";
import { useNavigation } from "@/composables/useNavigation";
import type { SidebarButton as SidebarButtonType } from "./types";

const { navigate, isActive } = useNavigation();

const buttons: SidebarButtonType[] = [
  { id: "editor", label: "Editor", icon: CodeXml },
  { id: "script-hub", label: "Script Hub", icon: Library },
  { id: "explorer", label: "Explorer", icon: PackageSearch },
  { id: "logs", label: "Logs", icon: Logs },
  { id: "settings", label: "Settings", icon: Settings },
];
</script>

<template>
  <div class="w-16 bg-app-shell flex flex-col items-center py-1 gap-2">
    <SidebarButton
      v-for="button in buttons"
      :key="button.id"
      :icon="button.icon"
      :label="button.label"
      :is-active="isActive(button.id)"
      @click="navigate(button.id)"
    />
  </div>
</template>
