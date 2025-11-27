<script setup lang="ts">
import { X, Minus, Square } from "lucide-vue-next";
import LogoText from "@/assets/logo-text.svg";
import Icon from "@/assets/icon.svg";
</script>

<template>
    <div class="h-12 bg-muted/50 flex items-center justify-between px-4">
        <div class="flex items-center gap-2">
            <img
                :src="Icon"
                alt="Riptide Icon"
                class="h-6 brightness-0 invert"
            />
            <img
                :src="LogoText"
                alt="Riptide"
                class="h-4 brightness-0 invert"
            />
        </div>
        <div class="flex items-center gap-2">
            <button
                class="h-6 w-6 flex items-center justify-center hover:bg-muted rounded transition-colors"
            >
                <Minus :size="16" />
            </button>
            <button
                class="h-6 w-6 flex items-center justify-center hover:bg-muted rounded transition-colors"
            >
                <Square :size="14" />
            </button>
            <button
                class="h-6 w-6 flex items-center justify-center hover:bg-destructive hover:text-destructive-foreground rounded transition-colors"
            >
                <X :size="16" />
            </button>
        </div>
    </div>
</template>
