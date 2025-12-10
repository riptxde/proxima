<script setup lang="ts">
import { ref, watch } from "vue";
import { Dock, DockIcon } from "@/components/ui/dock";
import { User, Search } from "lucide-vue-next";
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

const clientsDialogOpen = ref(false);
const searchQueryDialogOpen = ref(false);
const searchResultsDialogOpen = ref(false);
const dockTooltipKey = ref(0);

const handleClientsClick = () => {
  clientsDialogOpen.value = true;
};

const handleSearchClick = () => {
  searchQueryDialogOpen.value = true;
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
              <DockIcon @click="handleSearchClick">
                <Search
                  class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                />
              </DockIcon>
            </TooltipTrigger>
            <TooltipContent :side-offset="-15">
              <p>Search</p>
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
