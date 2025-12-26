<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from "vue";
import { Play, Square, Trash2, Code2, Scroll, Pause } from "lucide-vue-next";
import { Dock, DockIcon } from "@/components/ui/dock";
import LiquidGlass from "@/components/shared/LiquidGlass.vue";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import { useRemoteSpy } from "../composables/useRemoteSpy";
import { useEditorTabs } from "@/features/editor/composables/useEditorTabs";
import { useNavigation } from "@/composables/useNavigation";
import { toast } from "vue-sonner";
import ClientsDialog from "./ClientsDialog.vue";

const {
    isSpyActive,
    isPaused,
    selectedRemote,
    selectedCall,
    selectedClient,
    remotes,
    stopSpy,
    togglePause,
    clearCalls,
    generateCode,
    decompileScript,
} = useRemoteSpy();
const { openFileAsTab } = useEditorTabs();
const { navigate } = useNavigation();

const showClientDialog = ref(false);
const dockTooltipKey = ref(0);

const isCallSelected = computed(() => selectedCall.value !== null);
const hasCallingScript = computed(
    () => isCallSelected.value && !!selectedCall.value?.callingScriptPath,
);
const hasCalls = computed(() => remotes.value.length > 0);

const handleStartOrPause = () => {
    if (!isSpyActive.value) {
        // Start spy - show client selection dialog
        showClientDialog.value = true;
    } else {
        // Toggle pause
        togglePause();
        if (isPaused.value) {
            toast.info("Remote spy paused", {
                description: "Not receiving new remote calls",
            });
        } else {
            toast.info("Remote spy resumed", {
                description: "Receiving new remote calls",
            });
        }
    }
};

const handleStopSpy = async () => {
    const username = selectedClient.value?.username;
    try {
        await stopSpy();
        toast.success("Remote spy stopped", {
            description: `User: ${username}`,
        });
    } catch (error) {
        toast.error("Failed to stop remote spy", {
            description: String(error),
        });
    }
};

const handleSendCodeToEditor = async () => {
    if (!isCallSelected.value || !selectedRemote.value || !selectedCall.value) {
        toast.error("Could not send code", {
            description: "No remote call selected",
        });
        return;
    }

    const call = selectedCall.value;

    try {
        await generateCode(call.id);
    } catch (error) {
        toast.error("Failed to generate code", {
            description: String(error),
        });
    }
};

const handleDecompile = async () => {
    if (!hasCallingScript.value || !selectedCall.value?.callingScriptPath) {
        toast.error("Could not decompile", {
            description: "No calling script available",
        });
        return;
    }

    try {
        await decompileScript(selectedCall.value.callingScriptPath);
    } catch (error) {
        toast.error("Failed to decompile script", {
            description: String(error),
        });
    }
};

const handleClearCalls = () => {
    if (!hasCalls.value) {
        toast.error("Could not clear calls", {
            description: "No calls to clear",
        });
        return;
    }
    const count = clearCalls();
    toast.success("All calls cleared", {
        description: `${count} call${count !== 1 ? "s" : ""} removed`,
    });
};

// Remount dock tooltips when dialog closes
// This is absolutely necessary otherwise, tooltips stop working after a dialog opens
watch(showClientDialog, (newValue, oldValue) => {
    if (oldValue && !newValue) {
        dockTooltipKey.value++;
    }
});

// Listen for generated code and send to editor
const handleCodeGenerated = (event: Event) => {
    const customEvent = event as CustomEvent<{ callId: number; code: string }>;
    const { code } = customEvent.detail;

    if (!selectedRemote.value) return;

    try {
        openFileAsTab(`${selectedRemote.value.name} Call`, code);
        navigate("editor");
        toast.success("Code sent to editor", {
            description: `${selectedRemote.value.name} calling code`,
        });
    } catch (error) {
        toast.error("Failed to send code to editor", {
            description: String(error),
        });
    }
};

// Listen for decompiled code and send to editor
const handleDecompiled = (event: Event) => {
    const customEvent = event as CustomEvent<{
        scriptPath: string;
        source: string;
    }>;
    const { scriptPath, source } = customEvent.detail;

    // Extract script name from path
    const scriptName = scriptPath.split(".").pop() || "Script";

    try {
        openFileAsTab(`${scriptName} (Decompiled)`, source);
        navigate("editor");
        toast.success("Decompiled code sent to editor", {
            description: `Script: ${scriptName}`,
        });
    } catch (error) {
        toast.error("Failed to send decompiled code to editor", {
            description: String(error),
        });
    }
};

onMounted(() => {
    window.addEventListener("remote-spy-code-generated", handleCodeGenerated);
    window.addEventListener("remote-spy-decompiled", handleDecompiled);
});

onUnmounted(() => {
    window.removeEventListener(
        "remote-spy-code-generated",
        handleCodeGenerated,
    );
    window.removeEventListener("remote-spy-decompiled", handleDecompiled);
});
</script>

<template>
    <div class="p-4 flex items-center justify-center">
        <LiquidGlass>
            <TooltipProvider :key="dockTooltipKey">
                <Dock class="m-0!">
                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon @click="handleStartOrPause">
                                <Play
                                    v-if="!isSpyActive || isPaused"
                                    class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                                />
                                <Pause
                                    v-else
                                    class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>
                                {{
                                    !isSpyActive
                                        ? "Start Remote Spy"
                                        : isPaused
                                          ? "Resume Remote Spy"
                                          : "Pause Remote Spy"
                                }}
                            </p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon
                                @click="handleStopSpy"
                                :class="{
                                    'opacity-30 cursor-not-allowed':
                                        !isSpyActive,
                                }"
                            >
                                <Square
                                    class="size-5 text-app-shell-foreground transition-opacity"
                                    :class="{
                                        'opacity-60 group-hover:opacity-100':
                                            isSpyActive,
                                        'opacity-30': !isSpyActive,
                                    }"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Stop Remote Spy</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon
                                @click="handleSendCodeToEditor"
                                :class="{
                                    'opacity-30 cursor-not-allowed':
                                        !isCallSelected,
                                }"
                            >
                                <Code2
                                    class="size-5 text-app-shell-foreground transition-opacity"
                                    :class="{
                                        'opacity-60 group-hover:opacity-100':
                                            isCallSelected,
                                        'opacity-30': !isCallSelected,
                                    }"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Generate Calling Code</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon
                                @click="handleDecompile"
                                :class="{
                                    'opacity-30 cursor-not-allowed':
                                        !hasCallingScript,
                                }"
                            >
                                <Scroll
                                    class="size-5 text-app-shell-foreground transition-opacity"
                                    :class="{
                                        'opacity-60 group-hover:opacity-100':
                                            hasCallingScript,
                                        'opacity-30': !hasCallingScript,
                                    }"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Decompile Calling Script</p>
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon
                                @click="handleClearCalls"
                                :class="{
                                    'opacity-30 cursor-not-allowed': !hasCalls,
                                }"
                            >
                                <Trash2
                                    class="size-5 text-app-shell-foreground transition-opacity"
                                    :class="{
                                        'opacity-60 group-hover:opacity-100':
                                            hasCalls,
                                        'opacity-30': !hasCalls,
                                    }"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Clear All Calls</p>
                        </TooltipContent>
                    </Tooltip>
                </Dock>
            </TooltipProvider>
        </LiquidGlass>

        <!-- Client Selection Dialog -->
        <ClientsDialog v-model:open="showClientDialog" />
    </div>
</template>
