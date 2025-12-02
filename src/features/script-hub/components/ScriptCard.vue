<script setup lang="ts">
import {
    Eye,
    Send,
    Info,
    CheckCircle,
    KeyRound,
    Globe,
    AlertCircle,
} from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import GlowingEffect from "@/components/ui/glowing-effect/GlowingEffect.vue";
import type { Script } from "../types/script";

interface Props {
    script: Script;
}

defineProps<Props>();

const getImageUrl = (url: string) => {
    if (url.startsWith("http")) {
        return url;
    }
    return `https://scriptblox.com${url}`;
};
</script>

<template>
    <div
        class="rounded-xl relative h-full border-2 border-white/10 p-1 md:p-1.5"
    >
        <GlowingEffect
            :spread="40"
            :glow="true"
            :disabled="false"
            :proximity="64"
            :inactive-zone="0.01"
        />
        <div
            class="group relative h-full bg-card rounded-[7px] overflow-hidden shadow-[0px_0px_27px_0px_#2D2D2D] transition-all duration-300 flex flex-col"
        >
            <div
                class="relative aspect-video overflow-hidden bg-muted shrink-0"
            >
                <img
                    :src="getImageUrl(script.image)"
                    :alt="script.title"
                    class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
                    loading="lazy"
                />

                <div
                    class="absolute top-2 right-2 flex flex-wrap gap-1.5 justify-end max-w-[calc(100%-1rem)]"
                >
                    <span
                        v-if="script.verified"
                        class="px-2 py-1 rounded-md bg-green-500/60 backdrop-blur-sm border border-green-500/50 flex items-center gap-1 text-xs text-green-100"
                    >
                        <CheckCircle class="w-3 h-3" />
                        Verified
                    </span>

                    <span
                        v-if="script.isPatched"
                        class="px-2 py-1 rounded-md bg-red-500/60 backdrop-blur-sm border border-red-500/50 flex items-center gap-1 text-xs text-red-100"
                    >
                        <AlertCircle class="w-3 h-3" />
                        Patched
                    </span>

                    <span
                        v-if="script.isUniversal"
                        class="px-2 py-1 rounded-md bg-blue-500/60 backdrop-blur-sm border border-blue-500/50 flex items-center gap-1 text-xs text-blue-100"
                    >
                        <Globe class="w-3 h-3" />
                        Universal
                    </span>

                    <span
                        v-if="script.key"
                        class="px-2 py-1 rounded-md bg-yellow-500/60 backdrop-blur-sm border border-yellow-500/50 flex items-center gap-1 text-xs text-yellow-100"
                    >
                        <KeyRound class="w-3 h-3" />
                        Key
                    </span>

                    <span
                        class="px-2 py-1 rounded-md backdrop-blur-sm border flex items-center gap-1 text-xs"
                        :class="
                            script.scriptType === 'paid'
                                ? 'bg-purple-500/60 border-purple-500/50 text-purple-100'
                                : 'bg-emerald-500/60 border-emerald-500/50 text-emerald-100'
                        "
                    >
                        {{ script.scriptType === "paid" ? "Paid" : "Free" }}
                    </span>
                </div>

                <div
                    class="absolute bottom-2 left-2 px-2 py-1 rounded-md bg-black/60 backdrop-blur-sm border border-white/10 flex items-center gap-1.5 text-xs text-muted-foreground"
                >
                    <Eye class="w-3 h-3" />
                    {{ script.views.toLocaleString() }}
                </div>
            </div>

            <div class="p-4 flex flex-col flex-1">
                <div class="space-y-2 flex-1">
                    <h3
                        class="font-semibold text-foreground line-clamp-2 leading-tight group-hover:text-sidebar-primary transition-colors"
                    >
                        {{ script.title }}
                    </h3>

                    <p
                        class="text-xs text-muted-foreground font-mono bg-muted/50 px-2 py-1 rounded-md w-fit"
                    >
                        {{
                            script.isUniversal
                                ? "Universal Script"
                                : script.game.name
                        }}
                    </p>
                </div>

                <TooltipProvider>
                    <div class="flex gap-2 mt-3">
                        <Tooltip>
                            <TooltipTrigger as-child>
                                <Button
                                    size="sm"
                                    variant="outline"
                                    class="flex-1 h-9"
                                >
                                    <Send class="w-4 h-4" />
                                </Button>
                            </TooltipTrigger>
                            <TooltipContent side="bottom">
                                <p>Send to Editor</p>
                            </TooltipContent>
                        </Tooltip>

                        <Tooltip>
                            <TooltipTrigger as-child>
                                <Button
                                    size="sm"
                                    variant="outline"
                                    class="w-9 h-9 p-0"
                                >
                                    <Info class="w-4 h-4" />
                                </Button>
                            </TooltipTrigger>
                            <TooltipContent side="bottom">
                                <p>View Details</p>
                            </TooltipContent>
                        </Tooltip>
                    </div>
                </TooltipProvider>
            </div>
        </div>
    </div>
</template>
