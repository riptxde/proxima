<script setup lang="ts">
import { computed, ref } from "vue";
import { ChevronRight, ChevronDown } from "lucide-vue-next";
import { useExplorer } from "../composables/useExplorer";
import { useExplorerIcons } from "../composables/useExplorerIcons";
import type { ExplorerItem as ExplorerItemType } from "../types/explorer";

defineOptions({
  name: "ExplorerItem",
});

interface Props {
  item: ExplorerItemType;
  depth?: number;
}

const props = withDefaults(defineProps<Props>(), {
  depth: 0,
});

const { expandedIds, selectedItemId, getProperties, toggleExpand } =
  useExplorer();
const { getIconUrl, getFallbackUrl, markIconFailed } = useExplorerIcons();

const isExpanded = computed(() => expandedIds.value.has(props.item.id));
const isSelected = computed(() => selectedItemId.value === props.item.id);
const iconUrl = ref(getIconUrl(props.item.className));

const handleToggleExpand = () => {
  if (props.item.hasChildren) {
    toggleExpand(props.item.id);
  }
};

const handleSelectItem = () => {
  getProperties(props.item.id, props.item.className, props.item.name);
};

const handleIconError = () => {
  markIconFailed(props.item.className);
  iconUrl.value = getFallbackUrl();
};
</script>

<template>
  <div class="select-none">
    <div
      class="flex items-center gap-1 px-2 py-1 hover:bg-accent/30 cursor-pointer rounded transition-colors"
      :class="{ 'bg-accent/50': isSelected }"
      :style="{ paddingLeft: `${depth * 16 + 8}px` }"
      @click="handleSelectItem"
    >
      <div
        class="w-4 h-4 flex items-center justify-center shrink-0"
        @click.stop="handleToggleExpand"
      >
        <ChevronRight
          v-if="item.hasChildren && !isExpanded"
          class="w-3.5 h-3.5 text-muted-foreground"
        />
        <ChevronDown
          v-if="item.hasChildren && isExpanded"
          class="w-3.5 h-3.5 text-muted-foreground"
        />
      </div>
      <div class="flex items-center gap-2 min-w-0 flex-1">
        <img
          :src="iconUrl"
          :alt="item.className"
          class="w-4 h-4 shrink-0 object-contain"
          @error="handleIconError"
        />
        <span class="text-sm truncate">{{ item.name }}</span>
      </div>
    </div>

    <div v-if="isExpanded && item.hasChildren">
      <ExplorerItem
        v-for="child in item.children"
        :key="child.id"
        :item="child"
        :depth="depth + 1"
      />
    </div>
  </div>
</template>
