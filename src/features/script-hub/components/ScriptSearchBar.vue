<script setup lang="ts">
import { ref, watch } from "vue";
import {
    Search,
    SlidersHorizontal,
    X,
    RotateCcw,
    DollarSign,
    Gift,
    CheckCircle2,
    XCircle,
    KeyRound,
    Globe,
    ShieldAlert,
    Calendar,
    Clock,
    Eye,
    ThumbsUp,
    ThumbsDown,
    Target,
    ArrowDown,
    ArrowUp,
    Grid3x3,
    Scroll,
    Shield,
    Sparkles,
    AlertCircle,
    ArrowUpDown,
} from "lucide-vue-next";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import type { ScriptSearchParams } from "../types/script";

interface Props {
    modelValue: string;
    onSearch: (query: string) => void;
    filters: ScriptSearchParams;
    onFilterChange: (filters: Partial<ScriptSearchParams>) => void;
    onResetFilters: () => void;
}

const props = defineProps<Props>();
const emit = defineEmits<{
    "update:modelValue": [value: string];
}>();

const localValue = ref(props.modelValue);
const localFilters = ref<ScriptSearchParams>({ ...props.filters });

watch(
    () => props.modelValue,
    (newVal) => {
        localValue.value = newVal;
    },
);

watch(
    () => props.filters,
    (newFilters) => {
        localFilters.value = { ...newFilters };
    },
    { deep: true },
);

const handleInput = (value: string | number) => {
    const stringValue = String(value);
    localValue.value = stringValue;
    emit("update:modelValue", stringValue);
};

const handleSearch = () => {
    props.onSearch(localValue.value);
};

const handleClear = () => {
    localValue.value = "";
    emit("update:modelValue", "");
    props.onSearch("");
};

const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === "Enter") {
        handleSearch();
    }
};

const updateFilter = (key: keyof ScriptSearchParams, value: any) => {
    localFilters.value = {
        ...localFilters.value,
        [key]: value === "all" ? undefined : value,
    };
    props.onFilterChange({ [key]: value === "all" ? undefined : value });
};
</script>

<template>
    <div class="flex gap-2">
        <div class="relative flex-1">
            <Search
                class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground pointer-events-none"
            />
            <Input
                :model-value="localValue"
                @update:model-value="handleInput"
                @keydown="handleKeydown"
                placeholder="Search scripts..."
                class="pl-9 pr-9 h-10 bg-muted/50 border-border focus-visible:ring-sidebar-primary"
            />
            <button
                v-if="localValue"
                @click="handleClear"
                class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
                aria-label="Clear search"
            >
                <X class="w-4 h-4" />
            </button>
        </div>

        <Popover>
            <PopoverTrigger as-child>
                <Button
                    variant="outline"
                    size="default"
                    class="gap-2 min-w-[110px] h-10"
                >
                    <SlidersHorizontal class="w-4 h-4" />
                    Filters
                </Button>
            </PopoverTrigger>
            <PopoverContent class="w-[360px]" align="end">
                <div class="-m-4 select-none">
                    <div
                        class="flex items-center justify-between px-3 py-1.5 bg-tab-bar rounded-t-md"
                    >
                        <h3 class="text-sm font-medium text-foreground">
                            Filters
                        </h3>
                        <Button
                            @click="onResetFilters"
                            variant="ghost"
                            size="sm"
                            class="gap-1.5 h-8 text-muted-foreground hover:text-foreground"
                        >
                            <RotateCcw class="w-3.5 h-3.5" />
                            Reset
                        </Button>
                    </div>

                    <div class="grid grid-cols-2 gap-3 p-4">
                        <div class="space-y-1.5">
                            <label
                                class="text-xs text-muted-foreground flex items-center gap-1.5"
                            >
                                <Scroll class="w-3 h-3" />
                                Script Type
                            </label>
                            <Select
                                :model-value="localFilters.mode || 'all'"
                                @update:model-value="
                                    (val) => updateFilter('mode', val)
                                "
                            >
                                <SelectTrigger class="h-9 bg-muted/50 w-full">
                                    <SelectValue placeholder="All" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="all">
                                        <div class="flex items-center gap-2">
                                            <Grid3x3 class="w-3.5 h-3.5" />
                                            All
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="free">
                                        <div class="flex items-center gap-2">
                                            <Gift class="w-3.5 h-3.5" />
                                            Free
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="paid">
                                        <div class="flex items-center gap-2">
                                            <DollarSign class="w-3.5 h-3.5" />
                                            Paid
                                        </div>
                                    </SelectItem>
                                </SelectContent>
                            </Select>
                        </div>

                        <div class="space-y-1.5">
                            <label
                                class="text-xs text-muted-foreground flex items-center gap-1.5"
                            >
                                <Shield class="w-3 h-3" />
                                Verified
                            </label>
                            <Select
                                :model-value="
                                    localFilters.verified !== undefined
                                        ? String(localFilters.verified)
                                        : 'all'
                                "
                                @update:model-value="
                                    (val) =>
                                        updateFilter(
                                            'verified',
                                            val === 'all'
                                                ? undefined
                                                : Number(val),
                                        )
                                "
                            >
                                <SelectTrigger class="h-9 bg-muted/50 w-full">
                                    <SelectValue placeholder="All" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="all">
                                        <div class="flex items-center gap-2">
                                            <Grid3x3 class="w-3.5 h-3.5" />
                                            All
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="1">
                                        <div class="flex items-center gap-2">
                                            <CheckCircle2 class="w-3.5 h-3.5" />
                                            Yes
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="0">
                                        <div class="flex items-center gap-2">
                                            <XCircle class="w-3.5 h-3.5" />
                                            No
                                        </div>
                                    </SelectItem>
                                </SelectContent>
                            </Select>
                        </div>

                        <div class="space-y-1.5">
                            <label
                                class="text-xs text-muted-foreground flex items-center gap-1.5"
                            >
                                <KeyRound class="w-3 h-3" />
                                Key System
                            </label>
                            <Select
                                :model-value="
                                    localFilters.key !== undefined
                                        ? String(localFilters.key)
                                        : 'all'
                                "
                                @update:model-value="
                                    (val) =>
                                        updateFilter(
                                            'key',
                                            val === 'all'
                                                ? undefined
                                                : Number(val),
                                        )
                                "
                            >
                                <SelectTrigger class="h-9 bg-muted/50 w-full">
                                    <SelectValue placeholder="All" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="all">
                                        <div class="flex items-center gap-2">
                                            <Grid3x3 class="w-3.5 h-3.5" />
                                            All
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="1">
                                        <div class="flex items-center gap-2">
                                            <KeyRound class="w-3.5 h-3.5" />
                                            Yes
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="0">
                                        <div class="flex items-center gap-2">
                                            <XCircle class="w-3.5 h-3.5" />
                                            No
                                        </div>
                                    </SelectItem>
                                </SelectContent>
                            </Select>
                        </div>

                        <div class="space-y-1.5">
                            <label
                                class="text-xs text-muted-foreground flex items-center gap-1.5"
                            >
                                <Globe class="w-3 h-3" />
                                Universal
                            </label>
                            <Select
                                :model-value="
                                    localFilters.universal !== undefined
                                        ? String(localFilters.universal)
                                        : 'all'
                                "
                                @update:model-value="
                                    (val) =>
                                        updateFilter(
                                            'universal',
                                            val === 'all'
                                                ? undefined
                                                : Number(val),
                                        )
                                "
                            >
                                <SelectTrigger class="h-9 bg-muted/50 w-full">
                                    <SelectValue placeholder="All" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="all">
                                        <div class="flex items-center gap-2">
                                            <Grid3x3 class="w-3.5 h-3.5" />
                                            All
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="1">
                                        <div class="flex items-center gap-2">
                                            <Globe class="w-3.5 h-3.5" />
                                            Yes
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="0">
                                        <div class="flex items-center gap-2">
                                            <XCircle class="w-3.5 h-3.5" />
                                            No
                                        </div>
                                    </SelectItem>
                                </SelectContent>
                            </Select>
                        </div>

                        <div class="space-y-1.5">
                            <label
                                class="text-xs text-muted-foreground flex items-center gap-1.5"
                            >
                                <AlertCircle class="w-3 h-3" />
                                Patched
                            </label>
                            <Select
                                :model-value="
                                    localFilters.patched !== undefined
                                        ? String(localFilters.patched)
                                        : 'all'
                                "
                                @update:model-value="
                                    (val) =>
                                        updateFilter(
                                            'patched',
                                            val === 'all'
                                                ? undefined
                                                : Number(val),
                                        )
                                "
                            >
                                <SelectTrigger class="h-9 bg-muted/50 w-full">
                                    <SelectValue placeholder="All" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="all">
                                        <div class="flex items-center gap-2">
                                            <Grid3x3 class="w-3.5 h-3.5" />
                                            All
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="1">
                                        <div class="flex items-center gap-2">
                                            <ShieldAlert class="w-3.5 h-3.5" />
                                            Yes
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="0">
                                        <div class="flex items-center gap-2">
                                            <CheckCircle2 class="w-3.5 h-3.5" />
                                            No
                                        </div>
                                    </SelectItem>
                                </SelectContent>
                            </Select>
                        </div>

                        <div class="space-y-1.5">
                            <label
                                class="text-xs text-muted-foreground flex items-center gap-1.5"
                            >
                                <Sparkles class="w-3 h-3" />
                                Sort By
                            </label>
                            <Select
                                :model-value="
                                    localFilters.sortBy || 'createdAt'
                                "
                                @update:model-value="
                                    (val) => updateFilter('sortBy', val)
                                "
                            >
                                <SelectTrigger class="h-9 bg-muted/50 w-full">
                                    <SelectValue />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="createdAt">
                                        <div class="flex items-center gap-2">
                                            <Calendar class="w-3.5 h-3.5" />
                                            Created At
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="updatedAt">
                                        <div class="flex items-center gap-2">
                                            <Clock class="w-3.5 h-3.5" />
                                            Updated At
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="views">
                                        <div class="flex items-center gap-2">
                                            <Eye class="w-3.5 h-3.5" />
                                            Views
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="likeCount">
                                        <div class="flex items-center gap-2">
                                            <ThumbsUp class="w-3.5 h-3.5" />
                                            Likes
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="dislikeCount">
                                        <div class="flex items-center gap-2">
                                            <ThumbsDown class="w-3.5 h-3.5" />
                                            Dislikes
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="accuracy">
                                        <div class="flex items-center gap-2">
                                            <Target class="w-3.5 h-3.5" />
                                            Accuracy
                                        </div>
                                    </SelectItem>
                                </SelectContent>
                            </Select>
                        </div>

                        <div class="space-y-1.5 col-span-2">
                            <label
                                class="text-xs text-muted-foreground flex items-center gap-1.5"
                            >
                                <ArrowUpDown class="w-3 h-3" />
                                Sort Order
                            </label>
                            <Select
                                :model-value="localFilters.order || 'desc'"
                                @update:model-value="
                                    (val) => updateFilter('order', val)
                                "
                            >
                                <SelectTrigger class="h-9 bg-muted/50 w-full">
                                    <SelectValue />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="desc">
                                        <div class="flex items-center gap-2">
                                            <ArrowDown class="w-3.5 h-3.5" />
                                            Descending
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="asc">
                                        <div class="flex items-center gap-2">
                                            <ArrowUp class="w-3.5 h-3.5" />
                                            Ascending
                                        </div>
                                    </SelectItem>
                                </SelectContent>
                            </Select>
                        </div>
                    </div>
                </div>
            </PopoverContent>
        </Popover>
    </div>
</template>
