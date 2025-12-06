<script setup lang="ts">
import { computed } from "vue";
import StarsBackground from "@/components/ui/bg-stars/StarsBackground.vue";
import { Card } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Separator } from "@/components/ui/separator";
import { Slider } from "@/components/ui/slider";
import { Input } from "@/components/ui/input";
import { CodeXml, Play, Settings } from "lucide-vue-next";
import { useSettings } from "../composables/useSettings";

const { editorSettings, executionSettings, applicationSettings } =
    useSettings();

// Convert fontSize to/from array for Slider component
const fontSize = computed({
    get: () => [editorSettings.value.fontSize],
    set: (value) => {
        editorSettings.value.fontSize = value[0] ?? 14;
    },
});
</script>

<template>
    <div
        class="h-full overflow-hidden flex flex-col relative bg-card rounded-lg border border-border shadow-sm"
    >
        <!-- Stars Background -->
        <div class="absolute inset-0 z-0 pointer-events-none rounded-lg">
            <StarsBackground :factor="0.05" :speed="50" star-color="#fff" />
        </div>

        <div class="relative z-10 flex-1 overflow-y-auto px-4">
            <div class="max-w-4xl mx-auto space-y-4 my-4">
                <!-- Editor Settings -->
                <Card class="p-4">
                    <div class="space-y-3">
                        <div class="space-y-0.5 select-none">
                            <div class="flex items-center gap-2">
                                <CodeXml class="w-4 h-4" />
                                <h2 class="text-base font-semibold">Editor</h2>
                            </div>
                            <p class="text-xs text-muted-foreground">
                                Configure code editor behavior and appearance
                            </p>
                        </div>

                        <div class="space-y-2">
                            <!-- Word Wrap -->
                            <div
                                class="flex items-center justify-between py-1.5"
                            >
                                <div class="space-y-0 select-none">
                                    <Label
                                        for="word-wrap"
                                        class="text-sm cursor-pointer"
                                    >
                                        Word Wrap
                                    </Label>
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Wrap long lines to fit the editor width
                                    </p>
                                </div>
                                <Switch
                                    id="word-wrap"
                                    v-model="editorSettings.wordWrap"
                                />
                            </div>

                            <Separator />

                            <!-- Minimap -->
                            <div
                                class="flex items-center justify-between py-1.5"
                            >
                                <div class="space-y-0 select-none">
                                    <Label
                                        for="minimap"
                                        class="text-sm cursor-pointer"
                                    >
                                        Minimap
                                    </Label>
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Show code overview minimap
                                    </p>
                                </div>
                                <Switch
                                    id="minimap"
                                    v-model="editorSettings.minimap"
                                />
                            </div>

                            <Separator />

                            <!-- Font Size -->
                            <div class="space-y-2 py-1.5">
                                <div
                                    class="flex items-center justify-between select-none"
                                >
                                    <div class="space-y-0">
                                        <Label class="text-sm">
                                            Font Size
                                        </Label>
                                        <p
                                            class="text-xs text-muted-foreground font-normal"
                                        >
                                            Editor font size in pixels
                                        </p>
                                    </div>
                                    <span
                                        class="text-xs font-medium text-muted-foreground"
                                    >
                                        {{ fontSize[0] }}px
                                    </span>
                                </div>
                                <Slider
                                    v-model="fontSize"
                                    :min="10"
                                    :max="24"
                                    :step="1"
                                    class="w-full"
                                />
                            </div>

                            <Separator />

                            <!-- Font -->
                            <div class="space-y-2 py-1.5">
                                <div class="space-y-0 select-none">
                                    <Label for="font" class="text-sm">
                                        Font
                                    </Label>
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Editor font family
                                    </p>
                                </div>
                                <Input
                                    id="font"
                                    v-model="editorSettings.font"
                                    placeholder="Cascadia Code"
                                    class="w-full"
                                />
                            </div>

                            <Separator />

                            <!-- Font Ligatures -->
                            <div
                                class="flex items-center justify-between py-1.5"
                            >
                                <div class="space-y-0 select-none">
                                    <Label
                                        for="font-ligatures"
                                        class="text-sm cursor-pointer"
                                    >
                                        Font Ligatures
                                    </Label>
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Enable font ligatures for code symbols
                                    </p>
                                </div>
                                <Switch
                                    id="font-ligatures"
                                    v-model="editorSettings.fontLigatures"
                                />
                            </div>

                            <Separator />

                            <!-- Smooth Cursor -->
                            <div
                                class="flex items-center justify-between py-1.5"
                            >
                                <div class="space-y-0 select-none">
                                    <Label
                                        for="smooth-cursor"
                                        class="text-sm cursor-pointer"
                                    >
                                        Smooth Cursor
                                    </Label>
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Enable smooth cursor movement animation
                                    </p>
                                </div>
                                <Switch
                                    id="smooth-cursor"
                                    v-model="editorSettings.smoothCursor"
                                />
                            </div>

                            <Separator />

                            <!-- Smooth Cursor Blink -->
                            <div
                                class="flex items-center justify-between py-1.5"
                            >
                                <div class="space-y-0 select-none">
                                    <Label
                                        for="smooth-cursor-blink"
                                        class="text-sm cursor-pointer"
                                    >
                                        Smooth Cursor Blink
                                    </Label>
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Enable smooth cursor blinking animation
                                    </p>
                                </div>
                                <Switch
                                    id="smooth-cursor-blink"
                                    v-model="editorSettings.smoothCursorBlink"
                                />
                            </div>
                        </div>
                    </div>
                </Card>

                <!-- Execution Settings -->
                <Card class="p-4">
                    <div class="space-y-3">
                        <div class="space-y-0.5 select-none">
                            <div class="flex items-center gap-2">
                                <Play class="w-4 h-4" />
                                <h2 class="text-base font-semibold">
                                    Execution
                                </h2>
                            </div>
                            <p class="text-xs text-muted-foreground">
                                Configure script execution behavior
                            </p>
                        </div>

                        <div class="space-y-2">
                            <!-- Auto Execute -->
                            <div
                                class="flex items-center justify-between py-1.5"
                            >
                                <div class="space-y-0 select-none">
                                    <Label
                                        for="auto-execute"
                                        class="text-sm cursor-pointer"
                                    >
                                        Auto Execute
                                    </Label>
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Automatically execute AutoExec scripts
                                        when clients attach
                                    </p>
                                </div>
                                <Switch
                                    id="auto-execute"
                                    v-model="executionSettings.autoExecute"
                                />
                            </div>

                            <Separator />

                            <!-- HTTP Request Execution -->
                            <div
                                class="flex items-center justify-between py-1.5"
                            >
                                <div class="space-y-0 select-none">
                                    <Label
                                        for="http-request-execution"
                                        class="text-sm cursor-pointer"
                                    >
                                        HTTP Request Execution
                                    </Label>
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Allow script execution from external
                                        HTTP requests
                                    </p>
                                </div>
                                <Switch
                                    id="http-request-execution"
                                    v-model="
                                        executionSettings.httpRequestExecution
                                    "
                                />
                            </div>
                        </div>
                    </div>
                </Card>

                <!-- Application Settings -->
                <Card class="p-4">
                    <div class="space-y-3">
                        <div class="space-y-0.5 select-none">
                            <div class="flex items-center gap-2">
                                <Settings class="w-4 h-4" />
                                <h2 class="text-base font-semibold">
                                    Application
                                </h2>
                            </div>
                            <p class="text-xs text-muted-foreground">
                                General application preferences
                            </p>
                        </div>

                        <div class="space-y-2">
                            <!-- Always on Top -->
                            <div
                                class="flex items-center justify-between py-1.5"
                            >
                                <div class="space-y-0 select-none">
                                    <Label
                                        for="always-on-top"
                                        class="text-sm cursor-pointer"
                                    >
                                        Always on Top
                                    </Label>
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Keep Proxima window above other windows
                                    </p>
                                </div>
                                <Switch
                                    id="always-on-top"
                                    v-model="applicationSettings.alwaysOnTop"
                                />
                            </div>
                        </div>
                    </div>
                </Card>
            </div>
        </div>
    </div>
</template>
