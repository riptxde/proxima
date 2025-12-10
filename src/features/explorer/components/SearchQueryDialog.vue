<script setup lang="ts">
import { ref, watch } from "vue";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { toast } from "vue-sonner";
import { useExplorer } from "../composables/useExplorer";

defineProps<{
  open: boolean;
}>();

const emit = defineEmits<{
  "update:open": [value: boolean];
  "results-ready": [];
}>();

const { search, selectedClient } = useExplorer();
const searchQuery = ref("");
const searchBy = ref("name");
const limit = ref(100);

// Reset form when dialog opens
watch(
  () => emit,
  () => {
    searchQuery.value = "";
    searchBy.value = "name";
    limit.value = 100;
  },
);

const handleSearch = async () => {
  if (!selectedClient.value) {
    toast.error("No client selected");
    return;
  }

  if (!searchQuery.value.trim()) {
    toast.error("Please enter a search query");
    return;
  }

  try {
    await search(searchQuery.value, searchBy.value, limit.value);
    emit("update:open", false);
    emit("results-ready");
  } catch (error) {
    toast.error(`Search failed: ${error}`);
  }
};
</script>

<template>
  <Dialog :open="open" @update:open="$emit('update:open', $event)">
    <DialogContent class="sm:max-w-[500px]">
      <DialogHeader>
        <DialogTitle>Search Explorer</DialogTitle>
        <DialogDescription>
          Search for instances in the explorer tree.
        </DialogDescription>
      </DialogHeader>

      <div class="space-y-4">
        <div class="space-y-2">
          <label class="text-sm font-medium">Search Query</label>
          <Input
            v-model="searchQuery"
            placeholder="Enter search term..."
            @keydown.enter="handleSearch"
          />
        </div>

        <div class="space-y-2">
          <label class="text-sm font-medium">Search By</label>
          <Select v-model="searchBy">
            <SelectTrigger>
              <SelectValue placeholder="Select search type" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="name">Name</SelectItem>
              <SelectItem value="classname">ClassName</SelectItem>
              <SelectItem value="both">Both</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="space-y-2">
          <label class="text-sm font-medium">Limit (Max 1000)</label>
          <Input
            v-model.number="limit"
            type="number"
            min="1"
            max="1000"
            placeholder="100"
          />
        </div>

        <div class="flex gap-2 pt-2">
          <Button
            variant="outline"
            class="flex-1"
            @click="$emit('update:open', false)"
          >
            Cancel
          </Button>
          <Button class="flex-1" @click="handleSearch"> Search </Button>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
