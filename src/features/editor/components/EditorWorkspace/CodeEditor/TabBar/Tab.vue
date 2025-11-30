<script setup lang="ts">
import { ref } from "vue";
import TabLabel from "./TabLabel.vue";
import TabEditInput from "./TabEditInput.vue";
import TabCloseButton from "./TabCloseButton.vue";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import type { TabProps, TabEvents } from "./types";

const props = defineProps<TabProps>();

const emit = defineEmits<TabEvents>();

const isEditing = ref(false);
const editingName = ref("");

const handleSelect = () => {
    if (!isEditing.value) {
        emit("select", props.id);
    }
};

const startEditing = () => {
    if (!props.isActive) return;

    isEditing.value = true;
    editingName.value = props.name;
};

const finishEditing = () => {
    if (editingName.value.trim()) {
        emit("rename", props.id, editingName.value.trim());
    }
    isEditing.value = false;
};

const cancelEditing = () => {
    isEditing.value = false;
};

const handleClose = () => {
    emit("close", props.id);
};
</script>

<template>
    <Tooltip>
        <TooltipTrigger as-child>
            <button
                @click="handleSelect"
                :class="[
                    'flex items-center gap-2 px-3 py-1.5 rounded transition-all duration-300 ease-in-out border',
                    isActive
                        ? 'border-border text-foreground'
                        : 'border-transparent text-muted-foreground hover:text-foreground',
                ]"
            >
                <TabEditInput
                    v-if="isEditing"
                    v-model="editingName"
                    :tab-id="id"
                    @confirm="finishEditing"
                    @cancel="cancelEditing"
                />
                <TabLabel
                    v-else
                    :name="name"
                    @dblclick="startEditing"
                />
                <TabCloseButton
                    v-if="showClose"
                    @click="handleClose"
                />
            </button>
        </TooltipTrigger>
        <TooltipContent v-if="filePath">
            <p class="text-xs">{{ filePath }}</p>
        </TooltipContent>
    </Tooltip>
</template>
