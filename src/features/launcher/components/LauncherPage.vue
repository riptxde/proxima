<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useSettings } from "@/features/settings/composables/useSettings";
import { useLogger } from "@/composables/useLogger";
import { useLauncherRegistration } from "@/features/launcher/composables/useLauncherRegistration";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import { Switch } from "@/components/ui/switch";
import { Rocket, CheckCircle2, XCircle } from "lucide-vue-next";
import { toast } from "vue-sonner";

const { launcherSettings } = useSettings();
const { addLog } = useLogger();
const { isCurrentLauncher, isChecking, checkRegistration } =
    useLauncherRegistration();
const isRegistering = ref(false);
const isLaunching = ref(false);

onMounted(() => {
    checkRegistration();
});

async function handleRegister() {
    isRegistering.value = true;

    try {
        await invoke("launcher_register");
        addLog("success", "Protocol handler registered successfully");
        toast.success("Protocol Registered", {
            description:
                "Proxima is now the default handler for roblox-player:// URLs",
        });
        await checkRegistration();
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

async function handleLaunch() {
    isLaunching.value = true;

    try {
        await invoke("launcher_launch");
        toast.success("Launching Roblox", {
            description: "Roblox is being launched",
        });
    } catch (error) {
        const errorMessage =
            error instanceof Error ? error.message : String(error);
        addLog("error", `Failed to launch Roblox: ${errorMessage}`);
        toast.error("Launch Failed", {
            description: errorMessage,
        });
    } finally {
        isLaunching.value = false;
    }
}
</script>

<template>
    <div
        data-page="launcher"
        class="h-full overflow-y-auto px-4 rounded-lg border border-border shadow-sm"
    >
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
                                <Label for="version-override" class="text-sm">
                                    Version Override
                                </Label>
                                <p
                                    class="text-xs text-muted-foreground font-normal"
                                >
                                    Leave empty to use latest version, or enter
                                    specific version hash
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

                        <!-- Multi-Instance Setting -->
                        <div class="flex items-center justify-between py-1.5">
                            <div class="space-y-0 select-none flex-1">
                                <Label for="multi-instance" class="text-sm">
                                    Multi-Instance
                                </Label>
                                <p
                                    class="text-xs text-muted-foreground font-normal"
                                >
                                    Allow multiple Roblox instances to run
                                    simultaneously
                                </p>
                            </div>
                            <Switch
                                id="multi-instance"
                                v-model="launcherSettings.multiInstance"
                            />
                        </div>

                        <Separator />

                        <!-- Cooldown Setting -->
                        <div class="space-y-2 py-1.5">
                            <div class="space-y-0 select-none">
                                <Label for="cooldown" class="text-sm">
                                    Launch Cooldown (seconds)
                                </Label>
                                <p
                                    class="text-xs text-muted-foreground font-normal"
                                >
                                    Delay between launches to prevent
                                    authentication errors when using alt
                                    managers (recommended: 60s)
                                </p>
                            </div>
                            <Input
                                id="cooldown"
                                v-model.number="launcherSettings.cooldown"
                                type="number"
                                min="0"
                                max="300"
                                placeholder="60"
                                class="w-full"
                            />
                        </div>

                        <Separator />

                        <!-- Launch Roblox -->
                        <div class="flex items-center justify-between py-1.5">
                            <div class="space-y-0 select-none">
                                <Label class="text-sm">Launch Roblox</Label>
                                <p
                                    class="text-xs text-muted-foreground font-normal"
                                >
                                    Launch Roblox directly without a game URI
                                </p>
                            </div>
                            <Button
                                @click="handleLaunch"
                                :disabled="isLaunching"
                                size="sm"
                            >
                                Launch
                            </Button>
                        </div>

                        <Separator />

                        <!-- Registration Status Indicator -->
                        <div class="py-2">
                            <div class="flex items-center gap-2">
                                <template v-if="isChecking">
                                    <div
                                        class="w-4 h-4 border-2 border-muted-foreground border-t-transparent rounded-full animate-spin"
                                    ></div>
                                    <span class="text-xs text-muted-foreground"
                                        >Checking registration...</span
                                    >
                                </template>
                                <template v-else-if="isCurrentLauncher">
                                    <CheckCircle2
                                        class="w-4 h-4 text-green-500"
                                    />
                                    <span
                                        class="text-xs text-green-500 font-medium"
                                        >Proxima is the current launcher</span
                                    >
                                </template>
                                <template v-else>
                                    <XCircle
                                        class="w-4 h-4 text-muted-foreground"
                                    />
                                    <span
                                        class="text-xs text-muted-foreground font-medium"
                                        >Proxima is not the current
                                        launcher</span
                                    >
                                </template>
                            </div>
                        </div>

                        <Separator />

                        <!-- Register Protocol -->
                        <div class="flex items-center justify-between py-1.5">
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
</template>
