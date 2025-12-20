<script setup lang="ts">
import { ArrowUp, ArrowDown, Zap, FunctionSquare } from "lucide-vue-next";
import type { RemoteDirection, RemoteType } from "../types/remote-spy";

interface Props {
  activeDirections: RemoteDirection[];
  activeTypes: RemoteType[];
}

interface Emits {
  (e: "toggle-direction", direction: RemoteDirection): void;
  (e: "toggle-type", type: RemoteType): void;
}

defineProps<Props>();
defineEmits<Emits>();

const isDirectionActive = (
  direction: RemoteDirection,
  activeDirections: RemoteDirection[],
) => {
  return activeDirections.includes(direction);
};

const isTypeActive = (type: RemoteType, activeTypes: RemoteType[]) => {
  return activeTypes.includes(type);
};
</script>

<template>
  <div class="px-3 py-2.5 border-b border-border/60 bg-muted/40">
    <div class="flex items-center justify-center gap-1">
      <!-- Outgoing Filter -->
      <button
        @click="$emit('toggle-direction', 'outgoing')"
        class="flex items-center gap-1.5 h-8 px-2.5 rounded-md border transition-all"
        :class="
          isDirectionActive('outgoing', activeDirections)
            ? 'bg-green-500/15 border-green-500/40 text-green-400'
            : 'bg-background/60 border-border/60 text-muted-foreground hover:bg-muted hover:border-border hover:text-foreground'
        "
      >
        <ArrowUp class="w-3.5 h-3.5" />
        <span class="text-xs font-medium">Out</span>
      </button>

      <!-- Incoming Filter -->
      <button
        @click="$emit('toggle-direction', 'incoming')"
        class="flex items-center gap-1.5 h-8 px-2.5 rounded-md border transition-all"
        :class="
          isDirectionActive('incoming', activeDirections)
            ? 'bg-blue-500/15 border-blue-500/40 text-blue-400'
            : 'bg-background/60 border-border/60 text-muted-foreground hover:bg-muted hover:border-border hover:text-foreground'
        "
      >
        <ArrowDown class="w-3.5 h-3.5" />
        <span class="text-xs font-medium">In</span>
      </button>

      <!-- RemoteEvent Filter -->
      <button
        @click="$emit('toggle-type', 'RemoteEvent')"
        class="flex items-center gap-1.5 h-8 px-2.5 rounded-md border transition-all"
        :class="
          isTypeActive('RemoteEvent', activeTypes)
            ? 'bg-yellow-500/15 border-yellow-500/40 text-yellow-400'
            : 'bg-background/60 border-border/60 text-muted-foreground hover:bg-muted hover:border-border hover:text-foreground'
        "
      >
        <Zap class="w-3.5 h-3.5" />
        <span class="text-xs font-medium">Event</span>
      </button>

      <!-- RemoteFunction Filter -->
      <button
        @click="$emit('toggle-type', 'RemoteFunction')"
        class="flex items-center gap-1.5 h-8 px-2.5 rounded-md border transition-all"
        :class="
          isTypeActive('RemoteFunction', activeTypes)
            ? 'bg-purple-500/15 border-purple-500/40 text-purple-400'
            : 'bg-background/60 border-border/60 text-muted-foreground hover:bg-muted hover:border-border hover:text-foreground'
        "
      >
        <FunctionSquare class="w-3.5 h-3.5" />
        <span class="text-xs font-medium">Function</span>
      </button>
    </div>
  </div>
</template>
