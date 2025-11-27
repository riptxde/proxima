<template>
    <button
        ref="fileRef"
        type="button"
        :disabled="!isSelectable"
        :class="[
            cn(
                'flex w-fit items-center gap-1.5 rounded-sm pr-1 text-sm duration-200 ease-in-out rtl:pl-1 rtl:pr-0 opacity-60 hover:opacity-100',
                isSelectable
                    ? 'cursor-pointer'
                    : 'cursor-not-allowed opacity-50',
                $props.class,
            ),
        ]"
        :dir="direction"
        @click="onClickHandler"
    >
        <Icon :name="fileIcon" size="19" />
        <span class="select-none">{{ name }}</span>
    </button>
</template>

<script lang="ts" setup>
import { cn } from "@/lib/utils";
import {
    type TreeContextProps,
    type FileProps,
    TREE_CONTEXT_SYMBOL,
} from "./index";
import { inject, toRefs } from "vue";
import Icon from "@/components/ui/Icon.vue";

const props = withDefaults(defineProps<FileProps>(), {
    isSelectable: true,
});

const { id, name, isSelectable } = toRefs(props);

const treeContext = inject<TreeContextProps>(TREE_CONTEXT_SYMBOL);
if (!treeContext) {
    throw new Error("[File] must be used inside <Tree>");
}

const { selectItem, direction, fileIcon } = treeContext;

function onClickHandler() {
    if (!isSelectable.value) return;
    selectItem(id.value);
}
</script>
