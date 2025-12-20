<script setup lang="ts">
import { computed } from "vue";
import { Play, Square, Trash2, Code2, Scroll } from "lucide-vue-next";
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

const {
  isSpyActive,
  selectedRemote,
  selectedCall,
  startSpy,
  stopSpy,
  clearCalls,
  generateCodeForCall,
} = useRemoteSpy();
const { openFileAsTab } = useEditorTabs();
const { navigate } = useNavigation();

const isCallSelected = computed(() => selectedCall.value !== null);
const hasCallingScript = computed(
  () => isCallSelected.value && !!selectedCall.value?.callingScriptPath,
);

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
</script>

<template>
  <div class="p-4 flex items-center justify-center">
    <LiquidGlass>
      <TooltipProvider>
        <Dock class="m-0!">
          <Tooltip>
            <TooltipTrigger as-child>
              <DockIcon
                @click="startSpy"
                :class="{ 'opacity-30 cursor-not-allowed': isSpyActive }"
              >
                <Play
                  class="size-5 text-app-shell-foreground transition-opacity"
                  :class="{
                    'opacity-60 group-hover:opacity-100': !isSpyActive,
                    'opacity-30': isSpyActive,
                  }"
                />
              </DockIcon>
            </TooltipTrigger>
            <TooltipContent :side-offset="-15">
              <p>Start Remote Spy</p>
            </TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger as-child>
              <DockIcon
                @click="stopSpy"
                :class="{ 'opacity-30 cursor-not-allowed': !isSpyActive }"
              >
                <Square
                  class="size-5 text-app-shell-foreground transition-opacity"
                  :class="{
                    'opacity-60 group-hover:opacity-100': isSpyActive,
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
                :class="{ 'opacity-30 cursor-not-allowed': !isCallSelected }"
              >
                <Code2
                  class="size-5 text-app-shell-foreground transition-opacity"
                  :class="{
                    'opacity-60 group-hover:opacity-100': isCallSelected,
                    'opacity-30': !isCallSelected,
                  }"
                />
              </DockIcon>
            </TooltipTrigger>
            <TooltipContent :side-offset="-15">
              <p>Send Code to Editor</p>
            </TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger as-child>
              <DockIcon
                @click="handleDecompile"
                :class="{ 'opacity-30 cursor-not-allowed': !hasCallingScript }"
              >
                <Scroll
                  class="size-5 text-app-shell-foreground transition-opacity"
                  :class="{
                    'opacity-60 group-hover:opacity-100': hasCallingScript,
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
              <DockIcon @click="clearCalls">
                <Trash2
                  class="size-5 text-app-shell-foreground opacity-60 group-hover:opacity-100 transition-opacity"
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
  </div>
</template>
