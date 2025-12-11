<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { Dock, DockIcon } from "@/components/ui/dock";
import { User, Search, Unplug } from "lucide-vue-next";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { TooltipProvider } from "@/components/ui/tooltip";
import ClientsDialog from "./ClientsDialog.vue";
import SearchQueryDialog from "./SearchQueryDialog.vue";
import SearchResultsDialog from "./SearchResultsDialog.vue";
import LiquidGlass from "@/components/shared/LiquidGlass.vue";
import { useExplorer } from "../composables/useExplorer";
import { useLogger } from "@/composables/useLogger";
import { toast } from "vue-sonner";

const { addLog } = useLogger();

const { selectedClient, stopExplorer } = useExplorer();

const clientsDialogOpen = ref(false);
const searchQueryDialogOpen = ref(false);
const searchResultsDialogOpen = ref(false);
const dockTooltipKey = ref(0);

const isClientSelected = computed(() => selectedClient.value !== null);

const handleClientsClick = () => {
  clientsDialogOpen.value = true;
};

const handleSearchClick = () => {
  if (!isClientSelected.value) {
    toast.error("Cannot search", {
      description: "No client connected to explorer",
    });
    return;
  }
  searchQueryDialogOpen.value = true;
};

const handleDisconnectClick = async () => {
  if (!isClientSelected.value) {
    toast.error("Cannot disconnect", {
      description: "No client connected to explorer",
    });
    return;
  }
  try {
    await stopExplorer();
  } catch (error) {
    addLog("error", `Failed to disconnect explorer: ${error}`);
  }
};

const handleSearchResultsReady = () => {
  searchResultsDialogOpen.value = true;
};

// Remount dock tooltips when dialog closes
// This is absolutely necessary otherwise, tooltips stop working after a dialog opens
watch(
  [clientsDialogOpen, searchQueryDialogOpen, searchResultsDialogOpen],
  (newValues, oldValues) => {
    const wasOpen = oldValues.some((val) => val);
    const isOpen = newValues.some((val) => val);
    if (wasOpen && !isOpen) {
      dockTooltipKey.value++;
    }
  },
);
</script>

<template>
  <div class="flex items-center justify-center p-4">
    <LiquidGlass>
      <TooltipProvider :key="dockTooltipKey">
        <Dock class="m-0!">
          <Tooltip>
            <TooltipTrigger as-child>
              <DockIcon @click="handleClientsClick">
                <User
                  class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                />
              </DockIcon>
            </TooltipTrigger>
            <TooltipContent :side-offset="-15">
              <p>Select Client</p>
            </TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger as-child>
              <DockIcon
                @click="handleSearchClick"
                :class="{
                  'opacity-30 cursor-not-allowed': !isClientSelected,
                }"
              >
                <Search
                  class="size-5 text-app-shell-foreground transition-opacity"
                  :class="{
                    'opacity-60 group-hover:opacity-100': isClientSelected,
                    'opacity-30': !isClientSelected,
                  }"
                />
              </DockIcon>
            </TooltipTrigger>
            <TooltipContent :side-offset="-15">
              <p>Search</p>
            </TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger as-child>
              <DockIcon
                @click="handleDisconnectClick"
                :class="{
                  'opacity-30 cursor-not-allowed': !isClientSelected,
                }"
              >
                <Unplug
                  class="size-5 text-app-shell-foreground transition-opacity"
                  :class="{
                    'opacity-60 group-hover:opacity-100': isClientSelected,
                    'opacity-30': !isClientSelected,
                  }"
                />
              </DockIcon>
            </TooltipTrigger>
            <TooltipContent :side-offset="-15">
              <p>Disconnect Explorer</p>
            </TooltipContent>
          </Tooltip>
        </Dock>
      </TooltipProvider>
    </LiquidGlass>

    <ClientsDialog v-model:open="clientsDialogOpen" />
    <SearchQueryDialog
      v-model:open="searchQueryDialogOpen"
      @results-ready="handleSearchResultsReady"
    />
    <SearchResultsDialog v-model:open="searchResultsDialogOpen" />
  </div>
</template>
