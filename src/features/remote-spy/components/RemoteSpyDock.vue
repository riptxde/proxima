<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { User, Unplug, Trash2, Code2, Scroll } from "lucide-vue-next";
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
    selectedRemote,
    selectedCall,
    remotes,
    stopSpy,
    clearCalls,
    generateCodeForCall,
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

const handleDisconnect = async () => {
    if (!isSpyActive.value) {
        toast.error("Could not disconnect", {
            description: "No client connected to remote spy",
        });
        return;
    }
    try {
        await stopSpy();
    } catch (error) {
        toast.error(`Failed to stop remote spy: ${error}`);
    }
};

const handleSendCodeToEditor = () => {
    if (!isCallSelected.value || !selectedRemote.value) {
        toast.error("Could not send code", {
            description: "No remote call selected",
        });
        return;
    }

    const remote = selectedRemote.value;
    const call = selectedCall.value!;
    const code = generateCodeForCall(remote, call);

    try {
        openFileAsTab(`${remote.name} Call`, code);
        navigate("editor");
        toast.success("Code sent to editor");
    } catch (error) {
        toast.error("Failed to send code to editor");
    }
};

const handleDecompile = () => {
    if (!hasCallingScript.value) {
        toast.error("Could not decompile", {
            description: "No calling script available",
        });
        return;
    }

    toast.info("Decompile functionality will be implemented soon");
};

const handleClearCalls = () => {
    if (!hasCalls.value) {
        toast.error("Could not clear calls", {
            description: "No calls to clear",
        });
        return;
    }
    clearCalls();
    toast.success("All calls cleared");
};

// Remount dock tooltips when dialog closes
// This is absolutely necessary otherwise, tooltips stop working after a dialog opens
watch(showClientDialog, (newValue, oldValue) => {
    if (oldValue && !newValue) {
        dockTooltipKey.value++;
    }
});
</script>

<template>
    <div class="p-4 flex items-center justify-center">
        <LiquidGlass>
            <TooltipProvider :key="dockTooltipKey">
                <Dock class="m-0!">
                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon @click="showClientDialog = true">
                                <User
                                    class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
                                />
                            </DockIcon>
                        </TooltipTrigger>
                        <TooltipContent :side-offset="-15">
                            <p>Select Remote Spy Client</p>
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
                            <p>Send Calling Code to Editor</p>
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

                    <Tooltip>
                        <TooltipTrigger as-child>
                            <DockIcon
                                @click="handleDisconnect"
                                :class="{
                                    'opacity-30 cursor-not-allowed':
                                        !isSpyActive,
                                }"
                            >
                                <Unplug
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
                            <p>Disconnect Remote Spy</p>
                        </TooltipContent>
                    </Tooltip>
                </Dock>
            </TooltipProvider>
        </LiquidGlass>

        <!-- Client Selection Dialog -->
        <ClientsDialog v-model:open="showClientDialog" />
    </div>
</template>
