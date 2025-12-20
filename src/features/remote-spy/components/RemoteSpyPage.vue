<script setup lang="ts">
import { computed, ref } from "vue";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Search,
  ArrowUp,
  ArrowDown,
  Zap,
  FunctionSquare,
  ChevronLeft,
  ChevronRight,
} from "lucide-vue-next";
import RemoteSpyDock from "./RemoteSpyDock.vue";
import { useRemoteSpy } from "../composables/useRemoteSpy";

const {
  remotes,
  selectedRemote,
  selectedCall,
  selectedRemoteId,
  selectedCallId,
  isSpyActive,
  filters,
  selectRemote,
  deselectRemote,
  selectCall,
  toggleDirectionFilter,
  toggleTypeFilter,
  setSearchFilter,
  getDirectionCount,
  getTypeCount,
} = useRemoteSpy();

// Pagination
const CALLS_PER_PAGE = 5;
const currentPage = ref(1);

const paginatedCalls = computed(() => {
  if (!selectedRemote.value) return [];
  const start = (currentPage.value - 1) * CALLS_PER_PAGE;
  const end = start + CALLS_PER_PAGE;
  return selectedRemote.value.calls.slice(start, end);
});

const totalPages = computed(() => {
  if (!selectedRemote.value) return 0;
  return Math.ceil(selectedRemote.value.calls.length / CALLS_PER_PAGE);
});

const canGoToPrevPage = computed(() => currentPage.value > 1);
const canGoToNextPage = computed(() => currentPage.value < totalPages.value);

const goToPrevPage = () => {
  if (canGoToPrevPage.value) currentPage.value--;
};

const goToNextPage = () => {
  if (canGoToNextPage.value) currentPage.value++;
};

// When selecting a remote, reset pagination
const handleSelectRemote = (remoteId: string) => {
  selectRemote(remoteId);
  currentPage.value = 1;
};

// When deselecting a remote, reset pagination
const handleDeselectRemote = () => {
  deselectRemote();
  currentPage.value = 1;
};

// Helper functions
const formatTime = (date: Date) => {
  return (
    date.toLocaleTimeString("en-US", {
      hour12: false,
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    }) +
    "." +
    String(date.getMilliseconds()).padStart(3, "0")
  );
};

const getDirectionIcon = (direction: "outgoing" | "incoming") => {
  return direction === "outgoing" ? ArrowUp : ArrowDown;
};

const getDirectionColor = (direction: "outgoing" | "incoming") => {
  return direction === "outgoing" ? "text-green-400" : "text-blue-400";
};

const getTypeIcon = (type: "RemoteEvent" | "RemoteFunction") => {
  return type === "RemoteEvent" ? Zap : FunctionSquare;
};

const getTypeColor = (type: "RemoteEvent" | "RemoteFunction") => {
  return type === "RemoteEvent" ? "text-yellow-400" : "text-purple-400";
};

const isDirectionActive = (direction: "outgoing" | "incoming") => {
  return filters.value.directions.includes(direction);
};

const isTypeActive = (type: "RemoteEvent" | "RemoteFunction") => {
  return filters.value.types.includes(type);
};

const getDirectionStats = (
  calls: Array<{ direction: "outgoing" | "incoming" }>,
) => {
  const outgoing = calls.filter((c) => c.direction === "outgoing").length;
  const incoming = calls.filter((c) => c.direction === "incoming").length;
  return { outgoing, incoming };
};
</script>

<template>
  <div
    class="h-full overflow-hidden flex flex-col rounded-lg border border-border shadow-sm"
  >
    <div class="flex-1 overflow-hidden p-4 pb-0 flex flex-col gap-4">
      <!-- Resizable Panels -->
      <ResizablePanelGroup direction="horizontal" class="flex-1 gap-4">
        <!-- Remotes List Panel -->
        <ResizablePanel :default-size="45" :min-size="30">
          <div
            class="h-full flex flex-col bg-card rounded-lg border border-border"
          >
            <!-- Filter Header -->
            <div class="px-3 py-2.5 border-b border-border/60 bg-muted/40">
              <div class="flex items-center justify-center gap-1">
                <!-- Outgoing Filter -->
                <button
                  @click="toggleDirectionFilter('outgoing')"
                  class="flex items-center gap-1.5 h-8 px-2.5 rounded-md border transition-all"
                  :class="
                    isDirectionActive('outgoing')
                      ? 'bg-green-500/15 border-green-500/40 text-green-400'
                      : 'bg-background/60 border-border/60 text-muted-foreground hover:bg-muted hover:border-border hover:text-foreground'
                  "
                >
                  <ArrowUp class="w-3.5 h-3.5" />
                  <span class="text-xs font-medium">Out</span>
                </button>

                <!-- Incoming Filter -->
                <button
                  @click="toggleDirectionFilter('incoming')"
                  class="flex items-center gap-1.5 h-8 px-2.5 rounded-md border transition-all"
                  :class="
                    isDirectionActive('incoming')
                      ? 'bg-blue-500/15 border-blue-500/40 text-blue-400'
                      : 'bg-background/60 border-border/60 text-muted-foreground hover:bg-muted hover:border-border hover:text-foreground'
                  "
                >
                  <ArrowDown class="w-3.5 h-3.5" />
                  <span class="text-xs font-medium">In</span>
                </button>

                <!-- RemoteEvent Filter -->
                <button
                  @click="toggleTypeFilter('RemoteEvent')"
                  class="flex items-center gap-1.5 h-8 px-2.5 rounded-md border transition-all"
                  :class="
                    isTypeActive('RemoteEvent')
                      ? 'bg-yellow-500/15 border-yellow-500/40 text-yellow-400'
                      : 'bg-background/60 border-border/60 text-muted-foreground hover:bg-muted hover:border-border hover:text-foreground'
                  "
                >
                  <Zap class="w-3.5 h-3.5" />
                  <span class="text-xs font-medium">Event</span>
                </button>

                <!-- RemoteFunction Filter -->
                <button
                  @click="toggleTypeFilter('RemoteFunction')"
                  class="flex items-center gap-1.5 h-8 px-2.5 rounded-md border transition-all"
                  :class="
                    isTypeActive('RemoteFunction')
                      ? 'bg-purple-500/15 border-purple-500/40 text-purple-400'
                      : 'bg-background/60 border-border/60 text-muted-foreground hover:bg-muted hover:border-border hover:text-foreground'
                  "
                >
                  <FunctionSquare class="w-3.5 h-3.5" />
                  <span class="text-xs font-medium">Function</span>
                </button>
              </div>
            </div>

            <!-- Content Area - Always Visible -->
            <div class="flex-1 overflow-hidden flex flex-col">
              <!-- Transition between remotes list and selected remote view -->
              <Transition
                mode="out-in"
                enter-active-class="transition-all duration-200 ease-out"
                leave-active-class="transition-all duration-200 ease-in"
                enter-from-class="opacity-0 translate-y-4"
                enter-to-class="opacity-100 translate-y-0"
                leave-from-class="opacity-100 translate-y-0"
                leave-to-class="opacity-0 -translate-y-4"
              >
                <!-- Remotes List View -->
                <div
                  v-if="!selectedRemote"
                  key="remotes-list"
                  class="flex-1 overflow-hidden flex flex-col"
                >
                  <!-- Search Bar - Always Visible -->
                  <div class="px-3 pt-3 pb-2 shrink-0">
                    <div class="relative">
                      <Search
                        class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground pointer-events-none"
                      />
                      <Input
                        :model-value="filters.search"
                        @update:model-value="
                          (val) => setSearchFilter(String(val))
                        "
                        placeholder="Search remotes..."
                        class="pl-9 pr-3 h-9 bg-background/60 border-border/60 focus-visible:ring-sidebar-primary text-sm"
                      />
                    </div>
                  </div>

                  <!-- Remotes List or Empty State -->
                  <div class="flex-1 overflow-y-auto px-3 pb-3">
                    <!-- Empty State -->
                    <div
                      v-if="remotes.length === 0"
                      class="h-full flex items-center justify-center text-center py-12"
                    >
                      <div class="text-muted-foreground">
                        <Zap class="w-12 h-12 mx-auto mb-3 opacity-30" />
                        <p class="text-sm">No remote calls captured</p>
                        <p class="text-xs mt-1">
                          Start the spy to begin monitoring
                        </p>
                      </div>
                    </div>

                    <!-- Remotes List -->
                    <div v-else class="space-y-2">
                      <div
                        v-for="remote in remotes"
                        :key="remote.id"
                        class="px-3 py-2.5 rounded-md transition-all border border-border/50 bg-muted/30 hover:bg-muted/50 hover:border-border cursor-pointer"
                        @click="handleSelectRemote(remote.id)"
                      >
                        <div class="flex items-center gap-2.5">
                          <!-- Type Icon -->
                          <component
                            :is="getTypeIcon(remote.type)"
                            class="w-4 h-4 shrink-0"
                            :class="getTypeColor(remote.type)"
                          />

                          <!-- Remote Name -->
                          <span
                            class="text-sm font-semibold truncate flex-1 min-w-0"
                            >{{ remote.name }}</span
                          >

                          <!-- Direction Stats -->
                          <div
                            v-if="getDirectionStats(remote.calls).outgoing > 0"
                            class="flex items-center gap-1 text-green-400/80"
                          >
                            <ArrowUp class="w-3 h-3" />
                            <span class="font-mono text-[11px]">{{
                              getDirectionStats(remote.calls).outgoing
                            }}</span>
                          </div>
                          <div
                            v-if="getDirectionStats(remote.calls).incoming > 0"
                            class="flex items-center gap-1 text-blue-400/80"
                          >
                            <ArrowDown class="w-3 h-3" />
                            <span class="font-mono text-[11px]">{{
                              getDirectionStats(remote.calls).incoming
                            }}</span>
                          </div>

                          <!-- Chevron -->
                          <ChevronRight
                            class="w-4 h-4 text-muted-foreground shrink-0"
                          />
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                <!-- Selected Remote View -->
                <div
                  v-else
                  key="selected-remote"
                  class="flex-1 overflow-hidden flex flex-col"
                >
                  <!-- Selected Remote Header -->
                  <div
                    class="px-3 py-3 border-b border-border/60 bg-muted/40 shrink-0"
                  >
                    <!-- Back Button -->
                    <Button
                      variant="ghost"
                      size="sm"
                      class="mb-2 h-7 px-1! -ml-1 text-muted-foreground hover:text-foreground"
                      @click="handleDeselectRemote"
                    >
                      <ChevronLeft class="w-4 h-4" />
                      <span class="text-xs">Back to Remotes</span>
                    </Button>

                    <!-- Remote Info -->
                    <div class="flex items-center gap-2.5">
                      <!-- Type Icon -->
                      <component
                        :is="getTypeIcon(selectedRemote.type)"
                        class="w-4 h-4 shrink-0"
                        :class="getTypeColor(selectedRemote.type)"
                      />

                      <!-- Remote Name -->
                      <span
                        class="text-sm font-semibold truncate flex-1 min-w-0"
                        >{{ selectedRemote.name }}</span
                      >

                      <!-- Direction Stats -->
                      <div
                        v-if="
                          getDirectionStats(selectedRemote.calls).outgoing > 0
                        "
                        class="flex items-center gap-1 text-green-400/80"
                      >
                        <ArrowUp class="w-3 h-3" />
                        <span class="font-mono text-[11px]">{{
                          getDirectionStats(selectedRemote.calls).outgoing
                        }}</span>
                      </div>
                      <div
                        v-if="
                          getDirectionStats(selectedRemote.calls).incoming > 0
                        "
                        class="flex items-center gap-1 text-blue-400/80"
                      >
                        <ArrowDown class="w-3 h-3" />
                        <span class="font-mono text-[11px]">{{
                          getDirectionStats(selectedRemote.calls).incoming
                        }}</span>
                      </div>
                    </div>
                  </div>

                  <!-- Calls List -->
                  <div class="flex-1 overflow-y-auto p-3">
                    <div class="space-y-1.5">
                      <div
                        v-for="call in paginatedCalls"
                        :key="call.id"
                        class="px-3 py-2 rounded-md transition-all border cursor-pointer"
                        :class="
                          selectedCallId === call.id
                            ? 'bg-blue-500/30 shadow-[0_0_0_1px_rgb(70_150_250/0.5)] hover:bg-blue-500/40 border-blue-500/50'
                            : 'bg-card/50 border-border/40 hover:bg-card hover:border-border/60'
                        "
                        @click="selectCall(call.id)"
                      >
                        <div class="flex items-center gap-2">
                          <!-- Direction Icon -->
                          <component
                            :is="getDirectionIcon(call.direction)"
                            class="w-3.5 h-3.5 shrink-0"
                            :class="getDirectionColor(call.direction)"
                          />

                          <!-- Call Info -->
                          <div class="flex-1 min-w-0 space-y-1">
                            <div
                              class="text-[11px] text-muted-foreground font-mono"
                            >
                              {{ formatTime(call.timestamp) }}
                            </div>
                            <div
                              v-if="call.arguments.length > 0"
                              class="text-[11px] text-muted-foreground/70 font-mono truncate"
                            >
                              {{
                                call.arguments.map((a) => a.value).join(", ")
                              }}
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>

                  <!-- Pagination Controls -->
                  <div
                    v-if="totalPages > 1"
                    class="px-3 py-2.5 border-t border-border/60 bg-muted/40 shrink-0 flex items-center justify-between"
                  >
                    <Button
                      variant="ghost"
                      size="sm"
                      class="h-7 px-1!"
                      :disabled="!canGoToPrevPage"
                      @click="goToPrevPage"
                    >
                      <ChevronLeft class="w-3.5 h-3.5" />
                      <span class="text-xs">Prev</span>
                    </Button>

                    <span class="text-xs text-muted-foreground font-mono">
                      Page {{ currentPage }} of {{ totalPages }}
                    </span>

                    <Button
                      variant="ghost"
                      size="sm"
                      class="h-7 px-1!"
                      :disabled="!canGoToNextPage"
                      @click="goToNextPage"
                    >
                      <span class="text-xs">Next</span>
                      <ChevronRight class="w-3.5 h-3.5" />
                    </Button>
                  </div>
                </div>
              </Transition>
            </div>
          </div>
        </ResizablePanel>

        <ResizableHandle with-handle />

        <!-- Details Panel -->
        <ResizablePanel :default-size="55" :min-size="30">
          <div
            class="h-full flex flex-col bg-card rounded-lg border border-border"
          >
            <!-- Content -->
            <div class="flex-1 overflow-y-auto">
              <!-- Empty State -->
              <div
                v-if="!selectedCall"
                class="h-full flex items-center justify-center text-center p-8"
              >
                <div class="text-muted-foreground">
                  <FunctionSquare class="w-12 h-12 mx-auto mb-3 opacity-30" />
                  <p class="text-sm">No call selected</p>
                  <p class="text-xs mt-1">Select a call to view details</p>
                </div>
              </div>

              <!-- Call Details -->
              <div v-else class="p-5 space-y-4">
                <!-- Header Section -->
                <div class="space-y-3">
                  <!-- Remote Name -->
                  <div>
                    <h2 class="text-xl font-bold tracking-tight">
                      {{ selectedRemote!.name }}
                    </h2>
                  </div>

                  <!-- Metadata Badges -->
                  <div class="flex items-center gap-2 flex-wrap">
                    <!-- Timestamp Badge -->
                    <div
                      class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md bg-muted/60 border border-border/40"
                    >
                      <span
                        class="text-[10px] text-muted-foreground/70 uppercase tracking-wider font-medium"
                        >Time</span
                      >
                      <span class="text-xs font-mono">{{
                        formatTime(selectedCall.timestamp)
                      }}</span>
                    </div>

                    <!-- Direction Badge -->
                    <div
                      class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md border"
                      :class="
                        selectedCall.direction === 'outgoing'
                          ? 'bg-green-500/10 border-green-500/30'
                          : 'bg-blue-500/10 border-blue-500/30'
                      "
                    >
                      <component
                        :is="getDirectionIcon(selectedCall.direction)"
                        class="w-3 h-3"
                        :class="getDirectionColor(selectedCall.direction)"
                      />
                      <span class="text-xs font-medium capitalize">{{
                        selectedCall.direction
                      }}</span>
                    </div>

                    <!-- Type Badge -->
                    <div
                      class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md border"
                      :class="
                        selectedRemote!.type === 'RemoteEvent'
                          ? 'bg-yellow-500/10 border-yellow-500/30'
                          : 'bg-purple-500/10 border-purple-500/30'
                      "
                    >
                      <component
                        :is="getTypeIcon(selectedRemote!.type)"
                        class="w-3 h-3"
                        :class="getTypeColor(selectedRemote!.type)"
                      />
                      <span class="text-xs font-medium">{{
                        selectedRemote!.type
                      }}</span>
                    </div>
                  </div>
                </div>

                <!-- Divider -->
                <div class="h-px bg-border/60"></div>

                <!-- Path Information -->
                <div class="space-y-3">
                  <!-- Remote Path -->
                  <div>
                    <div
                      class="text-[10px] text-muted-foreground/70 uppercase tracking-wider font-semibold mb-2"
                    >
                      Remote Path
                    </div>
                    <div
                      class="px-3 py-2.5 rounded-md border border-border/60 bg-muted/40"
                    >
                      <div
                        class="text-xs font-mono break-all leading-relaxed text-foreground/90"
                      >
                        {{ selectedRemote!.path }}
                      </div>
                    </div>
                  </div>

                  <!-- Calling Script -->
                  <div v-if="selectedCall.callingScript">
                    <div
                      class="text-[10px] text-muted-foreground/70 uppercase tracking-wider font-semibold mb-2"
                    >
                      Calling Script
                    </div>
                    <div
                      class="px-3 py-2.5 rounded-md border border-border/60 bg-muted/40"
                    >
                      <div
                        class="text-xs font-mono break-all leading-relaxed text-foreground/90"
                      >
                        {{
                          selectedCall.callingScriptPath ||
                          selectedCall.callingScript
                        }}
                      </div>
                    </div>
                  </div>
                </div>

                <!-- Arguments Section -->
                <div v-if="selectedCall.arguments.length > 0" class="space-y-3">
                  <div
                    class="text-[10px] text-muted-foreground/70 uppercase tracking-wider font-semibold"
                  >
                    Arguments ({{ selectedCall.arguments.length }})
                  </div>

                  <div class="space-y-2.5">
                    <div
                      v-for="(arg, index) in selectedCall.arguments"
                      :key="index"
                      class="group rounded-md border border-border/60 bg-muted/30 overflow-hidden transition-all hover:border-border hover:bg-muted/50"
                    >
                      <!-- Argument Header -->
                      <div
                        class="flex items-center justify-between px-3 py-2 bg-muted/50 border-b border-border/40"
                      >
                        <div class="flex items-center gap-2">
                          <span
                            class="text-xs font-mono font-semibold text-sidebar-primary"
                            >{{ index + 1 }}</span
                          >
                          <div class="w-px h-3.5 bg-border/60"></div>
                          <Badge
                            variant="secondary"
                            class="text-[10px] font-mono px-1.5 py-0 h-5 bg-background/60"
                          >
                            {{ arg.type }}
                          </Badge>
                        </div>
                      </div>

                      <!-- Argument Value -->
                      <div class="px-3 py-2.5">
                        <div
                          class="text-xs font-mono break-all leading-relaxed text-foreground/90"
                        >
                          {{ arg.value }}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                <!-- Return Value Section -->
                <div v-if="selectedCall.returnValue" class="space-y-3">
                  <div
                    class="text-[10px] text-muted-foreground/70 uppercase tracking-wider font-semibold"
                  >
                    Return Value
                  </div>

                  <div
                    class="group rounded-md border border-green-500/30 bg-green-500/5 overflow-hidden transition-all hover:border-green-500/50 hover:bg-green-500/10"
                  >
                    <!-- Return Header -->
                    <div
                      class="flex items-center justify-between px-3 py-2 bg-green-500/10 border-b border-green-500/20"
                    >
                      <div class="flex items-center gap-2">
                        <span
                          class="text-xs font-mono font-semibold text-green-400"
                          >return</span
                        >
                        <div class="w-px h-3.5 bg-green-500/30"></div>
                        <Badge
                          variant="secondary"
                          class="text-[10px] font-mono px-1.5 py-0 h-5 bg-background/60 border-green-500/20"
                        >
                          {{ selectedCall.returnValue.type }}
                        </Badge>
                      </div>
                    </div>

                    <!-- Return Value -->
                    <div class="px-3 py-2.5">
                      <div
                        class="text-xs font-mono break-all leading-relaxed text-foreground/90"
                      >
                        {{ selectedCall.returnValue.value }}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>

    <!-- Dock -->
    <RemoteSpyDock />
  </div>
</template>
