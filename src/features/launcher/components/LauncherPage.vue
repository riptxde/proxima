<script setup lang="ts">
import { ref } from "vue";
import { useSettings } from "@/features/settings/composables/useSettings";
import { useLogger } from "@/composables/useLogger";
import { useLauncherProgress } from "@/features/launcher/composables/useLauncherProgress";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import { Progress } from "@/components/ui/progress";
import { Rocket } from "lucide-vue-next";
import { toast } from "vue-sonner";

const { launcherSettings } = useSettings();
const { addLog } = useLogger();
const { isLaunching, launchProgress, launchStatus, launchError } =
    useLauncherProgress();
const isRegistering = ref(false);

async function handleRegister() {
    isRegistering.value = true;

    try {
        await invoke("launcher_register");
        addLog("success", "Protocol handler registered successfully");
        toast.success("Protocol Registered", {
            description:
                "Proxima is now the default handler for roblox-player:// URLs",
        });
    } catch (error) {
        const errorMessage =
            error instanceof Error ? error.message : String(error);
        addLog("error", `Failed to register protocol: ${errorMessage}`);
        toast.error("Registration Failed", {
            description: errorMessage,
        });
    } finally {
        isRegistering.value = false;
    }
}
</script>

<template>
    <div
        data-page="launcher"
        class="h-full overflow-hidden flex flex-col rounded-lg border border-border shadow-sm"
    >
        <!-- Launch Status Bar (always visible) -->
        <div
            class="border-b border-border px-4 py-3 space-y-2 transition-colors"
            :class="{
                'bg-card': !launchError,
                'bg-destructive/10': launchError,
            }"
        >
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <div
                        class="w-2 h-2 rounded-full transition-all"
                        :class="{
                            'bg-green-500 animate-pulse':
                                isLaunching && !launchError,
                            'bg-red-500': launchError,
                            'bg-muted-foreground': !isLaunching && !launchError,
                        }"
                    ></div>
                    <p
                        class="text-sm font-medium transition-colors"
                        :class="{ 'text-destructive': launchError }"
                    >
                        {{ launchError || launchStatus }}
                    </p>
                </div>
                <span
                    v-if="isLaunching && !launchError"
                    class="text-xs text-muted-foreground font-mono"
                >
                    {{ launchProgress }}%
                </span>
            </div>
            <Progress
                :model-value="launchProgress"
                class="h-1"
                :class="{
                    'opacity-30 animate-pulse': !isLaunching && !launchError,
                    'opacity-100': isLaunching || launchError,
                }"
            />
        </div>

        <!-- Content -->
        <div class="flex-1 overflow-y-auto px-4">
            <div class="max-w-4xl mx-auto space-y-4 my-4">
                <!-- Launcher Settings -->
                <Card class="p-4">
                    <div class="space-y-3">
                        <div class="space-y-0.5 select-none">
                            <div class="flex items-center gap-2">
                                <Rocket class="w-4 h-4" />
                                <h2 class="text-base font-semibold">
                                    Roblox Launcher
                                </h2>
                            </div>
                            <p class="text-xs text-muted-foreground">
                                Configure Roblox client version and protocol
                                handling
                            </p>
                        </div>

                        <div class="space-y-2">
                            <!-- Channel Setting -->
                            <div class="space-y-2 py-1.5">
                                <div class="space-y-0 select-none">
                                    <Label for="channel" class="text-sm"
                                        >Channel</Label
                                    >
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Leave empty to use the LIVE channel, we
                                        recommend leaving this field empty
                                    </p>
                                </div>
                                <Input
                                    id="channel"
                                    v-model="launcherSettings.channel"
                                    placeholder="Leave empty for LIVE"
                                    class="w-full"
                                />
                            </div>

                            <Separator />

                            <!-- Version Override Setting -->
                            <div class="space-y-2 py-1.5">
                                <div class="space-y-0 select-none">
                                    <Label
                                        for="version-override"
                                        class="text-sm"
                                    >
                                        Version Override
                                    </Label>
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Leave empty to use latest version, or
                                        enter specific version hash
                                    </p>
                                </div>
                                <Input
                                    id="version-override"
                                    v-model="launcherSettings.versionOverride"
                                    placeholder="Leave empty for latest"
                                    class="w-full"
                                />
                            </div>

                            <Separator />

                            <!-- Register Protocol -->
                            <div
                                class="flex items-center justify-between py-1.5"
                            >
                                <div class="space-y-0 select-none">
                                    <Label class="text-sm"
                                        >Protocol Registration</Label
                                    >
                                    <p
                                        class="text-xs text-muted-foreground font-normal"
                                    >
                                        Register Proxima as the handler for
                                        roblox-player:// URLs
                                    </p>
                                </div>
                                <Button
                                    @click="handleRegister"
                                    :disabled="isRegistering"
                                    size="sm"
                                >
                                    Register
                                </Button>
                            </div>
                        </div>
                    </div>
                </Card>
            </div>
        </div>
    </div>
</template>
