<script setup lang="ts">
import { ref, onMounted } from "vue";
import { ChevronLeft, ChevronRight, Loader2 } from "lucide-vue-next";
import StarsBackground from "@/components/ui/bg-stars/StarsBackground.vue";
import { Button } from "@/components/ui/button";
import ScriptCard from "./ScriptCard.vue";
import ScriptSearchBar from "./ScriptSearchBar.vue";
import { useScriptHub } from "../composables/useScriptHub";

const {
    scripts,
    isLoading,
    error,
    currentPage,
    totalPages,
    searchParams,
    fetchScripts,
    updateSearchParams,
    setPage,
    setSearch,
    resetFilters,
    hasNextPage,
    hasPrevPage,
} = useScriptHub();

const searchQuery = ref("");

onMounted(() => {
    fetchScripts();
});

const handleSearch = (query: string) => {
    setSearch(query);
};

const handleFilterChange = (filters: any) => {
    updateSearchParams(filters);
};

const handleResetFilters = () => {
    searchQuery.value = "";
    resetFilters();
};

const handlePrevPage = () => {
    if (hasPrevPage.value) {
        setPage(currentPage.value - 1);
    }
};

const handleNextPage = () => {
    if (hasNextPage.value) {
        setPage(currentPage.value + 1);
    }
};
</script>

<template>
    <div
        class="h-full overflow-hidden flex flex-col relative bg-card rounded-lg border border-border shadow-sm"
    >
        <div class="absolute inset-0 z-0 pointer-events-none rounded-lg">
            <StarsBackground :factor="0.05" :speed="50" star-color="#fff" />
        </div>

        <div class="relative z-10 h-full flex flex-col">
            <div
                class="shrink-0 p-6 space-y-4 border-b border-border bg-card/80"
            >
                <div>
                    <h1 class="text-2xl font-semibold text-foreground">
                        Script Hub
                    </h1>
                    <p class="text-sm text-muted-foreground mt-1">
                        Browse and find a vast collection of scripts
                    </p>
                </div>

                <ScriptSearchBar
                    v-model="searchQuery"
                    :on-search="handleSearch"
                    :filters="searchParams"
                    :on-filter-change="handleFilterChange"
                    :on-reset-filters="handleResetFilters"
                />
            </div>

            <div class="flex-1 overflow-y-auto p-6">
                <div
                    v-if="isLoading"
                    class="flex items-center justify-center h-64"
                >
                    <div class="flex flex-col items-center gap-3">
                        <Loader2
                            class="w-8 h-8 animate-spin text-sidebar-primary"
                        />
                        <p class="text-sm text-muted-foreground">
                            Loading scripts...
                        </p>
                    </div>
                </div>

                <div
                    v-else-if="error"
                    class="flex items-center justify-center h-64"
                >
                    <div class="text-center space-y-2">
                        <p class="text-destructive font-medium">
                            Failed to load scripts
                        </p>
                        <p class="text-sm text-muted-foreground">{{ error }}</p>
                        <Button @click="fetchScripts" size="sm" class="mt-4">
                            Try Again
                        </Button>
                    </div>
                </div>

                <div
                    v-else-if="scripts.length === 0"
                    class="flex items-center justify-center h-64"
                >
                    <div class="text-center space-y-2">
                        <p class="text-foreground font-medium">
                            No scripts found
                        </p>
                        <p class="text-sm text-muted-foreground">
                            Try adjusting your search or filters
                        </p>
                    </div>
                </div>

                <div v-else class="space-y-6">
                    <div
                        class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4"
                    >
                        <ScriptCard
                            v-for="script in scripts"
                            :key="script._id"
                            :script="script"
                        />
                    </div>

                    <div
                        class="flex items-center justify-between pt-4 border-t border-border"
                    >
                        <div class="text-sm text-muted-foreground">
                            Page {{ currentPage }} of
                            {{ totalPages.toLocaleString() }}
                        </div>

                        <div class="flex items-center gap-2">
                            <Button
                                @click="handlePrevPage"
                                :disabled="!hasPrevPage"
                                variant="outline"
                                size="sm"
                                class="gap-1"
                            >
                                <ChevronLeft class="w-4 h-4" />
                                Previous
                            </Button>

                            <Button
                                @click="handleNextPage"
                                :disabled="!hasNextPage"
                                variant="outline"
                                size="sm"
                                class="gap-1"
                            >
                                Next
                                <ChevronRight class="w-4 h-4" />
                            </Button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
