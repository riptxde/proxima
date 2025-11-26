<script setup lang="ts">
import {
    ResizablePanelGroup,
    ResizablePanel,
    ResizableHandle,
} from "@/components/ui/resizable";
import TitleBar from "@/components/TitleBar.vue";
import Sidebar from "@/components/Sidebar.vue";
import ScriptEditor from "@/components/ScriptEditor.vue";
import FileExplorer from "@/components/FileExplorer.vue";
import Dock from "@/components/Dock.vue";
</script>

<template>
    <div class="h-screen w-screen flex flex-col bg-background">
        <!-- Top Bar -->
        <TitleBar />

        <!-- Main Content Area -->
        <div class="flex-1 overflow-hidden flex">
            <!-- Sidebar -->
            <Sidebar />

            <!-- Content Area with Panels and Dock -->
            <div class="flex-1 overflow-hidden flex flex-col">
                <!-- Main Panel Grid -->
                <div class="flex-1 overflow-hidden p-4">
                    <ResizablePanelGroup
                        direction="horizontal"
                        class="h-full gap-4"
                    >
                        <!-- Script Editor -->
                        <ResizablePanel :default-size="70" :min-size="30">
                            <ScriptEditor />
                        </ResizablePanel>

                        <ResizableHandle with-handle />

                        <!-- File Tree Panel Card -->
                        <ResizablePanel :default-size="25" :min-size="20">
                            <FileExplorer />
                        </ResizablePanel>
                    </ResizablePanelGroup>
                </div>

                <!-- Dock spans everything except sidebar -->
                <Dock />
            </div>
        </div>
    </div>
</template>
