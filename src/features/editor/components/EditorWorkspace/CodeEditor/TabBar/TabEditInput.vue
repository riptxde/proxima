<script setup lang="ts">
import { ref, onMounted, watch } from "vue";

interface Props {
    modelValue: string;
    tabId: number;
}

const props = defineProps<Props>();

const emit = defineEmits<{
    "update:modelValue": [value: string];
    confirm: [];
    cancel: [];
}>();

const inputRef = ref<HTMLInputElement | null>(null);
const inputWidth = ref(96);

const measureTextWidth = (text: string) => {
    const canvas = document.createElement("canvas");
    const context = canvas.getContext("2d");
    if (!context) return 96;
    context.font = "14px ui-sans-serif, system-ui, sans-serif";
    return Math.min(context.measureText(text).width + 4, 96);
};

const updateInputWidth = () => {
    inputWidth.value = measureTextWidth(props.modelValue);
};

const handleInput = (event: Event) => {
    const target = event.target as HTMLInputElement;
    emit("update:modelValue", target.value);
    updateInputWidth();
};

watch(
    () => props.modelValue,
    () => {
        updateInputWidth();
    },
);

onMounted(() => {
    updateInputWidth();
    inputRef.value?.select();
});
</script>

<template>
    <input
        ref="inputRef"
        :value="modelValue"
        @input="handleInput"
        @blur="$emit('confirm')"
        @keydown.enter="$emit('confirm')"
        @keydown.esc="$emit('cancel')"
        @click.stop
        :data-tab-id="tabId"
        :style="{ width: inputWidth + 'px' }"
        class="text-sm bg-transparent border-none outline-none"
    />
</template>
