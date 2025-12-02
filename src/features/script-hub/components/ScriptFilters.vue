<script setup lang="ts">
import { ref, watch } from 'vue';
import { RotateCcw } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import type { ScriptSearchParams, ScriptMode, SortBy, SortOrder } from '../types/script';

interface Props {
  filters: ScriptSearchParams;
  onFilterChange: (filters: Partial<ScriptSearchParams>) => void;
  onReset: () => void;
}

const props = defineProps<Props>();

const localFilters = ref<ScriptSearchParams>({ ...props.filters });

watch(() => props.filters, (newFilters) => {
  localFilters.value = { ...newFilters };
}, { deep: true });

const updateFilter = (key: keyof ScriptSearchParams, value: any) => {
  localFilters.value = {
    ...localFilters.value,
    [key]: value === 'all' ? undefined : value,
  };
  props.onFilterChange({ [key]: value === 'all' ? undefined : value });
};
</script>

<template>
  <div class="bg-card border border-border rounded-xl p-4 space-y-4">
    <div class="flex items-center justify-between">
      <h3 class="text-sm font-medium text-foreground">Filters</h3>
      <Button
        @click="onReset"
        variant="ghost"
        size="sm"
        class="gap-1.5 h-8 text-muted-foreground hover:text-foreground"
      >
        <RotateCcw class="w-3.5 h-3.5" />
        Reset
      </Button>
    </div>

    <div class="grid grid-cols-2 gap-3">
      <div class="space-y-1.5">
        <label class="text-xs text-muted-foreground">Script Type</label>
        <Select
          :model-value="localFilters.mode || 'all'"
          @update:model-value="(val) => updateFilter('mode', val)"
        >
          <SelectTrigger class="h-9 bg-muted/50">
            <SelectValue placeholder="All" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All</SelectItem>
            <SelectItem value="free">Free</SelectItem>
            <SelectItem value="paid">Paid</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="space-y-1.5">
        <label class="text-xs text-muted-foreground">Verified</label>
        <Select
          :model-value="localFilters.verified !== undefined ? String(localFilters.verified) : 'all'"
          @update:model-value="(val) => updateFilter('verified', val === 'all' ? undefined : Number(val))"
        >
          <SelectTrigger class="h-9 bg-muted/50">
            <SelectValue placeholder="All" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All</SelectItem>
            <SelectItem value="1">Yes</SelectItem>
            <SelectItem value="0">No</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="space-y-1.5">
        <label class="text-xs text-muted-foreground">Key System</label>
        <Select
          :model-value="localFilters.key !== undefined ? String(localFilters.key) : 'all'"
          @update:model-value="(val) => updateFilter('key', val === 'all' ? undefined : Number(val))"
        >
          <SelectTrigger class="h-9 bg-muted/50">
            <SelectValue placeholder="All" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All</SelectItem>
            <SelectItem value="1">Yes</SelectItem>
            <SelectItem value="0">No</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="space-y-1.5">
        <label class="text-xs text-muted-foreground">Universal</label>
        <Select
          :model-value="localFilters.universal !== undefined ? String(localFilters.universal) : 'all'"
          @update:model-value="(val) => updateFilter('universal', val === 'all' ? undefined : Number(val))"
        >
          <SelectTrigger class="h-9 bg-muted/50">
            <SelectValue placeholder="All" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All</SelectItem>
            <SelectItem value="1">Yes</SelectItem>
            <SelectItem value="0">No</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="space-y-1.5">
        <label class="text-xs text-muted-foreground">Patched</label>
        <Select
          :model-value="localFilters.patched !== undefined ? String(localFilters.patched) : 'all'"
          @update:model-value="(val) => updateFilter('patched', val === 'all' ? undefined : Number(val))"
        >
          <SelectTrigger class="h-9 bg-muted/50">
            <SelectValue placeholder="All" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All</SelectItem>
            <SelectItem value="1">Yes</SelectItem>
            <SelectItem value="0">No</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="space-y-1.5">
        <label class="text-xs text-muted-foreground">Sort By</label>
        <Select
          :model-value="localFilters.sortBy || 'createdAt'"
          @update:model-value="(val) => updateFilter('sortBy', val)"
        >
          <SelectTrigger class="h-9 bg-muted/50">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="createdAt">Created At</SelectItem>
            <SelectItem value="updatedAt">Updated At</SelectItem>
            <SelectItem value="views">Views</SelectItem>
            <SelectItem value="likeCount">Likes</SelectItem>
            <SelectItem value="dislikeCount">Dislikes</SelectItem>
            <SelectItem value="accuracy">Accuracy</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="space-y-1.5">
        <label class="text-xs text-muted-foreground">Sort Order</label>
        <Select
          :model-value="localFilters.order || 'desc'"
          @update:model-value="(val) => updateFilter('order', val)"
        >
          <SelectTrigger class="h-9 bg-muted/50">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="desc">Descending</SelectItem>
            <SelectItem value="asc">Ascending</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>
  </div>
</template>
