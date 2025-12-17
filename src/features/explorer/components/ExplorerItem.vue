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
  parentPath?: string;
}

const props = withDefaults(defineProps<Props>(), {
  depth: 0,
  parentPath: "",
});

const {
  expandedIds,
  selectedItemId,
  selectedProperty,
  expGetProperties,
  toggleExpand,
} = useExplorer();
const { getIconUrl, getFallbackUrl, markIconFailed } = useExplorerIcons();

const isExpanded = computed(() => expandedIds.value.has(props.item.id));
const isSelected = computed(
  () => selectedItemId.value === props.item.id && !selectedProperty.value,
);
const iconUrl = ref(getIconUrl(props.item.className));

// Compute the path string for this item
const pathString = computed(() => {
  if (!props.parentPath) {
    // Root level - check if it's Workspace
    if (props.item.className === "Workspace") {
      return "workspace";
    }
    // Otherwise use game:GetService
    return `game:GetService("${props.item.className}")`;
  }

  // Check if name needs bracket notation
  const needsBrackets = !/^[A-Za-z_][A-Za-z0-9_]*$/.test(props.item.name);

  if (needsBrackets) {
    // Escape the string literal
    const hasSingleQuote = props.item.name.includes("'");
    const hasDoubleQuote = props.item.name.includes('"');

    let escapedName: string;
    if (hasSingleQuote && hasDoubleQuote) {
      escapedName =
        '"' + props.item.name.replace(/\\/g, "\\\\").replace(/"/g, '\\"') + '"';
    } else if (hasDoubleQuote) {
      escapedName = "'" + props.item.name + "'";
    } else {
      escapedName = '"' + props.item.name + '"';
    }

    return `${props.parentPath}[${escapedName}]`;
  } else {
    return `${props.parentPath}.${props.item.name}`;
  }
});

const handleToggleExpand = () => {
  if (props.item.hasChildren) {
    toggleExpand(props.item.id);
  }
};

const handleSelectItem = () => {
  expGetProperties(
    props.item.id,
    props.item.className,
    props.item.name,
    pathString.value,
  );
};

const handleIconError = () => {
  markIconFailed(props.item.className);
  iconUrl.value = getFallbackUrl();
};
</script>

<template>
  <div class="select-none">
    <div
      :data-explorer-item-id="item.id"
      class="flex items-center gap-1 px-2 py-1 cursor-pointer rounded transition-all"
      :class="
        isSelected
          ? 'bg-blue-500/30 shadow-[0_0_0_1px_rgb(70_150_250/0.5)] hover:bg-blue-500/40'
          : 'hover:bg-accent/30'
      "
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
        :parent-path="pathString"
      />
    </div>
  </div>
</template>
