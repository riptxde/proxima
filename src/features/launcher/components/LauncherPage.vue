<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useSettings } from "@/features/settings/composables/useSettings";
import { useLogger } from "@/composables/useLogger";
import { useLauncherRegistration } from "@/features/launcher/composables/useLauncherRegistration";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { Rocket, CheckCircle2 } from "lucide-vue-next";
import { toast } from "vue-sonner";

const { launcherSettings } = useSettings();
const { addLog } = useLogger();
const { isCurrentLauncher, isChecking, checkRegistration } =
    useLauncherRegistration();
const isRegistering = ref(false);
const isLaunching = ref(false);

const channelMode = ref<"LIVE" | "custom">("LIVE");
const customChannel = ref("");
const versionMode = ref<"latest" | "past" | "custom">("latest");
const customVersion = ref("");
const pastVersions = ref<{ version: string; date: string } | null>(null);
const isLoadingSettings = ref(true);

// Format date to show "X days ago"
const formatDate = (dateString: string) => {
    try {
        // Parse the date string (e.g., "12/18/2025, 6:22:24 PM UTC")
        const date = new Date(dateString);
        const now = new Date();
        const diffTime = Math.abs(now.getTime() - date.getTime());
        const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

        if (diffDays === 0) return "today";
        if (diffDays === 1) return "1 day ago";
        return `${diffDays} days ago`;
    } catch {
        return dateString;
    }
};

// Computed display value for version select (just show version hash for past)
const versionDisplayValue = computed(() => {
    if (versionMode.value === "latest") return "Latest Version";
    if (versionMode.value === "past" && pastVersions.value) {
        return pastVersions.value.version;
    }
    if (versionMode.value === "custom") return "Custom";
    return "";
});

// Compute actual values based on mode
const actualChannel = computed(() => {
    if (channelMode.value === "LIVE") return "";
    return customChannel.value;
});

const actualVersion = computed(() => {
    if (versionMode.value === "latest") return "";
    if (versionMode.value === "past" && pastVersions.value) {
        return pastVersions.value.version;
    }
    return customVersion.value;
});

// Sync computed values back to settings
const syncToSettings = () => {
    launcherSettings.value.channel = actualChannel.value;
    launcherSettings.value.versionOverride = actualVersion.value;
};

// Watch for changes and sync
const updateChannel = (value: any) => {
    if (!value || typeof value !== "string") return;
    channelMode.value = value as "LIVE" | "custom";
    syncToSettings();
};

const updateVersion = (value: any) => {
    if (!value || typeof value !== "string") return;
    versionMode.value = value as "latest" | "past" | "custom";
    syncToSettings();
};

const updateCustomChannel = () => {
    syncToSettings();
};

const updateCustomVersion = () => {
    syncToSettings();
};

// Initialize modes from settings
const initializeModes = () => {
    if (
        !launcherSettings.value.channel ||
        launcherSettings.value.channel === ""
    ) {
        channelMode.value = "LIVE";
        customChannel.value = "";
    } else {
        channelMode.value = "custom";
        customChannel.value = launcherSettings.value.channel;
    }

    if (
        !launcherSettings.value.versionOverride ||
        launcherSettings.value.versionOverride === ""
    ) {
        versionMode.value = "latest";
        customVersion.value = "";
    } else if (
        pastVersions.value &&
        launcherSettings.value.versionOverride === pastVersions.value.version
    ) {
        versionMode.value = "past";
    } else {
        versionMode.value = "custom";
        customVersion.value = launcherSettings.value.versionOverride;
    }
};

// Fetch past versions
const fetchPastVersions = async () => {
    try {
        const data = await invoke<{ Windows: string; WindowsDate: string }>(
            "launcher_fetch_past_versions",
        );
        pastVersions.value = {
            version: data.Windows,
            date: data.WindowsDate,
        };
    } catch (error) {
        console.error("Failed to fetch past versions:", error);
    }
};

onMounted(async () => {
    checkRegistration();
    await fetchPastVersions();
    initializeModes();
    isLoadingSettings.value = false;
});

async function handleRegister() {
    isRegistering.value = true;

    try {
        await invoke("launcher_register");
        addLog("success", "Protocol handler registered successfully");
        toast.success("Proxima is now your Roblox launcher", {
            description: "Clicking 'Play' on Roblox will now open Proxima",
        });
        await checkRegistration();
    } catch (error) {
        const errorMessage =
            error instanceof Error ? error.message : String(error);
        addLog("error", `Failed to register protocol: ${errorMessage}`);
        toast.error("Failed to set Proxima as launcher", {
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
        toast.success("Launching Roblox");
    } catch (error) {
        const errorMessage =
            error instanceof Error ? error.message : String(error);
        addLog("error", `Failed to launch Roblox: ${errorMessage}`);
        toast.error("Failed to launch", {
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
        class="h-full grid grid-cols-[2fr_3fr] gap-4 p-4 rounded-lg border border-border shadow-sm overflow-hidden"
    >
        <!-- Left Column - Actions -->
        <div class="flex flex-col gap-4">
            <!-- Launch Card -->
            <Card class="p-6 relative overflow-hidden flex-1 flex flex-col">
                <div
                    class="absolute inset-0 bg-linear-to-br from-primary/5 via-transparent to-transparent"
                ></div>
                <div
                    class="relative flex flex-col items-center justify-center flex-1 gap-4"
                >
                    <div
                        class="w-14 h-14 rounded-full bg-primary/10 flex items-center justify-center"
                    >
                        <Rocket class="w-7 h-7 text-primary" />
                    </div>
                    <div class="text-center space-y-1 select-none">
                        <h3 class="text-base font-semibold">Launch Roblox</h3>
                        <p class="text-xs text-muted-foreground">
                            Start Roblox from Proxima
                        </p>
                    </div>
                    <Button
                        @click="handleLaunch"
                        :disabled="isLaunching"
                        class="w-full"
                    >
                        <Rocket class="w-4 h-4 mr-2" />
                        {{ isLaunching ? "Launching..." : "Launch" }}
                    </Button>
                </div>
            </Card>

            <!-- Set as Launcher Card -->
            <Card
                class="p-6 relative overflow-hidden flex-1 flex flex-col"
                :class="isCurrentLauncher ? 'border-green-500/50' : ''"
            >
                <div
                    class="absolute inset-0"
                    :class="
                        isCurrentLauncher
                            ? 'bg-linear-to-br from-green-500/5 via-transparent to-transparent'
                            : 'bg-linear-to-br from-primary/5 via-transparent to-transparent'
                    "
                ></div>
                <div
                    class="relative flex flex-col items-center justify-center flex-1 gap-4"
                >
                    <div
                        class="w-14 h-14 rounded-full flex items-center justify-center"
                        :class="
                            isCurrentLauncher
                                ? 'bg-green-500/10'
                                : 'bg-primary/10'
                        "
                    >
                        <CheckCircle2
                            class="w-7 h-7"
                            :class="
                                isCurrentLauncher
                                    ? 'text-green-500'
                                    : 'text-primary'
                            "
                        />
                    </div>
                    <div class="text-center space-y-1 select-none">
                        <div class="flex items-center justify-center gap-2">
                            <h3 class="text-base font-semibold">
                                {{
                                    isCurrentLauncher
                                        ? "Active Launcher"
                                        : "Set as Launcher"
                                }}
                            </h3>
                            <template v-if="isChecking">
                                <div
                                    class="w-3 h-3 border-2 border-muted-foreground border-t-transparent rounded-full animate-spin"
                                ></div>
                            </template>
                            <template v-else-if="isCurrentLauncher">
                                <div
                                    class="w-2 h-2 rounded-full bg-green-500 animate-pulse"
                                ></div>
                            </template>
                        </div>
                        <p class="text-xs text-muted-foreground">
                            {{
                                isCurrentLauncher
                                    ? "Proxima is your launcher"
                                    : "Make Proxima your launcher"
                            }}
                        </p>
                    </div>
                    <Button
                        @click="handleRegister"
                        :disabled="isRegistering || isCurrentLauncher"
                        :variant="isCurrentLauncher ? 'outline' : 'default'"
                        class="w-full"
                    >
                        <CheckCircle2 class="w-4 h-4 mr-2" />
                        {{
                            isCurrentLauncher
                                ? "Active"
                                : isRegistering
                                  ? "Setting..."
                                  : "Use Proxima"
                        }}
                    </Button>
                </div>
            </Card>
        </div>

        <!-- Right Column - Settings -->
        <Card class="p-6 overflow-auto">
            <div
                v-if="isLoadingSettings"
                class="flex items-center justify-center h-full"
            >
                <div class="flex flex-col items-center gap-2">
                    <div
                        class="w-8 h-8 border-4 border-primary border-t-transparent rounded-full animate-spin"
                    ></div>
                    <p class="text-sm text-muted-foreground">
                        Loading launcher settings...
                    </p>
                </div>
            </div>
            <div v-else class="flex flex-col gap-6 h-full content-start">
                <!-- Multi-Instance -->
                <div class="space-y-2">
                    <div class="flex items-center justify-between">
                        <Label for="multi-instance" class="text-sm">
                            Multi-Instance
                        </Label>
                        <Switch
                            id="multi-instance"
                            v-model="launcherSettings.multiInstance"
                        />
                    </div>
                    <p class="text-xs text-muted-foreground">
                        Run multiple instances simultaneously
                    </p>
                </div>

                <!-- Cooldown -->
                <div class="space-y-2">
                    <Label for="cooldown" class="text-sm">
                        Launch Cooldown (seconds)
                    </Label>
                    <Input
                        id="cooldown"
                        v-model.number="launcherSettings.cooldown"
                        type="number"
                        min="0"
                        max="300"
                        placeholder="60"
                        autocomplete="off"
                        class="[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                    />
                    <p class="text-xs text-muted-foreground">
                        Specify a delay between launches, which can prevent
                        authentication errors when using Roblox alt managers
                    </p>
                </div>

                <!-- Channel -->
                <div class="space-y-2">
                    <Label for="channel" class="text-sm">Channel</Label>
                    <Select
                        :model-value="channelMode"
                        @update:model-value="updateChannel"
                    >
                        <SelectTrigger id="channel" class="w-full">
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="LIVE">LIVE</SelectItem>
                            <SelectItem value="custom">Custom</SelectItem>
                        </SelectContent>
                    </Select>
                    <Input
                        v-if="channelMode === 'custom'"
                        v-model="customChannel"
                        @input="updateCustomChannel"
                        placeholder="Enter custom channel"
                        autocomplete="off"
                        class="mt-2"
                    />
                    <p class="text-xs text-muted-foreground">
                        Specify the channel used for downloading Roblox
                        deployments
                    </p>
                </div>

                <!-- Version Override -->
                <div class="space-y-2">
                    <Label for="version-override" class="text-sm">
                        Version Override
                    </Label>
                    <Select
                        :model-value="versionMode"
                        @update:model-value="updateVersion"
                    >
                        <SelectTrigger id="version-override" class="w-full">
                            <SelectValue>
                                {{ versionDisplayValue }}
                            </SelectValue>
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="latest"
                                >Latest Version</SelectItem
                            >
                            <SelectItem v-if="pastVersions" value="past">
                                <span>{{ pastVersions.version }}</span>
                                <span class="text-muted-foreground ml-2"
                                    >({{ formatDate(pastVersions.date) }})</span
                                >
                            </SelectItem>
                            <SelectItem value="custom">Custom</SelectItem>
                        </SelectContent>
                    </Select>
                    <Input
                        v-if="versionMode === 'custom'"
                        v-model="customVersion"
                        @input="updateCustomVersion"
                        placeholder="Enter version hash"
                        autocomplete="off"
                        class="mt-2"
                    />
                    <p class="text-xs text-muted-foreground">
                        Use a specific version-hash to downgrade to or use a
                        specific Roblox version
                    </p>
                </div>
            </div>
        </Card>
    </div>
</template>
