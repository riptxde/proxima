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
import DockArea from "@/components/DockArea.vue";
import StarsBackground from "@/components/ui/bg-stars/StarsBackground.vue";
</script>

<template>
    <div class="h-screen w-screen flex flex-col bg-background">
        <!-- Top Bar -->
        <TitleBar />

        <!-- Main Content Area -->
        <div class="flex-1 overflow-hidden flex">
            <!-- Sidebar -->
            <Sidebar />

            <!-- Background matching app shell color -->
            <div class="flex-1 overflow-hidden bg-app-shell pr-2 pb-2">
                <!-- Content Area with rounded corners -->
                <div
                    class="h-full overflow-hidden flex flex-col relative bg-card rounded-lg border border-border shadow-sm"
                >
                    <div
                        class="absolute inset-0 z-0 pointer-events-none rounded-lg"
                    >
                        <StarsBackground
                            :factor="0.05"
                            :speed="50"
                            star-color="#fff"
                        />
                    </div>
                    <!-- Main Panel Grid -->
                    <div class="flex-1 overflow-hidden pt-4 px-4 relative z-10">
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
                    <div class="relative z-10">
                        <DockArea />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
