<script setup lang="ts">
import { ref, computed } from "vue";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Search } from "lucide-vue-next";
import { useExplorer } from "../composables/useExplorer";

defineProps<{
  open: boolean;
}>();

const emit = defineEmits<{
  "update:open": [value: boolean];
}>();

const {
  searchResults,
  searchQuery: originalQuery,
  searchLimited,
} = useExplorer();
const filterQuery = ref("");

const filteredResults = computed(() => {
  if (!filterQuery.value.trim()) {
    return searchResults.value;
  }

  const query = filterQuery.value.toLowerCase();
  return searchResults.value.filter(
    (result) =>
      result.name.toLowerCase().includes(query) ||
      result.className.toLowerCase().includes(query) ||
      result.pathString.toLowerCase().includes(query),
  );
});
</script>

<template>
  <Dialog :open="open" @update:open="$emit('update:open', $event)">
    <DialogContent class="sm:max-w-[700px] max-w-[90vw] overflow-hidden">
      <DialogHeader>
        <DialogTitle>Search Results</DialogTitle>
        <DialogDescription>
          Found {{ searchResults.length }} result{{
            searchResults.length !== 1 ? "s" : ""
          }}
          for "{{ originalQuery }}"
          <span v-if="searchLimited" class="text-yellow-500">(limited)</span>
        </DialogDescription>
      </DialogHeader>

      <div class="space-y-4 overflow-hidden">
        <div class="relative">
          <Search
            class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground"
          />
          <Input
            v-model="filterQuery"
            placeholder="Filter results..."
            class="pl-9"
          />
        </div>

        <div
          v-if="searchResults.length === 0"
          class="text-center py-12 text-muted-foreground"
        >
          <p class="text-sm">No results found</p>
        </div>

        <div
          v-else-if="filteredResults.length === 0"
          class="text-center py-12 text-muted-foreground"
        >
          <p class="text-sm">No results match your filter</p>
        </div>

        <div v-else class="rounded-md border overflow-hidden">
          <div class="max-h-[60vh] overflow-y-auto">
            <div
              v-for="result in filteredResults"
              :key="result.id"
              class="px-4 py-3 hover:bg-accent/50 transition-colors border-b last:border-b-0"
            >
              <div class="min-w-0 space-y-1.5">
                <div class="flex items-center gap-2 flex-wrap">
                  <span class="text-sm font-semibold">{{ result.name }}</span>
                  <span
                    class="text-xs px-1.5 py-0.5 rounded bg-blue-500/20 text-blue-400 font-mono"
                  >
                    {{ result.className }}
                  </span>
                  <span
                    class="text-xs px-1.5 py-0.5 rounded bg-muted/50 text-muted-foreground font-mono"
                  >
                    ID: {{ result.id }}
                  </span>
                </div>
                <div
                  class="text-xs text-muted-foreground font-mono truncate"
                  :title="result.pathString"
                >
                  {{ result.pathString }}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
