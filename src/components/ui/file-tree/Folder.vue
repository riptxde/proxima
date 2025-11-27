<template>
    <div class="relative h-full overflow-hidden">
        <div
            class="flex cursor-pointer items-center gap-1.5 rounded-md text-sm transition-all duration-200"
            :class="[
                cn(
                    'flex cursor-pointer items-center gap-1.5 rounded-md text-sm',
                    isSelect && isSelectable ? 'bg-muted' : '',
                    !isSelectable ? 'cursor-not-allowed opacity-50' : '',
                    $props.class,
                ),
            ]"
            :dir="direction"
            @click="onTriggerClick"
        >
            <Icon v-if="isExpanded" :name="openIcon" size="19" />
            <Icon v-else :name="closeIcon" size="19" />

            <span class="select-none">{{ name }}</span>
        </div>

        <div v-if="isExpanded" class="relative text-sm">
            <TreeIndicator v-if="name && indicator" aria-hidden="true" />
            <div
                class="ml-6 flex flex-col gap-1.5 py-1 rtl:mr-6"
                :dir="direction"
            >
                <slot />
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { cn } from "@/lib/utils";
import {
    type TreeContextProps,
    type FolderProps,
    TREE_CONTEXT_SYMBOL,
} from "./index";
import { inject, computed, toRefs } from "vue";
import Icon from "@/components/ui/Icon.vue";
import TreeIndicator from "./TreeIndicator.vue";

const props = withDefaults(defineProps<FolderProps>(), {
    isSelectable: true,
});

const { id, name, isSelectable, isSelect } = toRefs(props);

const treeContext = inject<TreeContextProps>(TREE_CONTEXT_SYMBOL);
if (!treeContext) {
    throw new Error("[Folder] must be used inside <Tree>");
}

const {
    expandedItems,
    handleExpand,
    openIcon,
    closeIcon,
    direction,
    indicator,
} = treeContext;

const isExpanded = computed<boolean>(() => {
    return !!expandedItems.value?.includes(id.value);
});

function onTriggerClick() {
    if (!isSelectable.value) return;
    handleExpand(id.value);
}
</script>
