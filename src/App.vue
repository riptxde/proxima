<script setup lang="ts">
import {
    ResizablePanelGroup,
    ResizablePanel,
    ResizableHandle,
} from "@/components/ui/resizable";
import TitleBar from "@/components/TitleBar.vue";
import Sidebar from "@/components/Sidebar.vue";
import ScriptEditor from "@/components/ScriptEditor.vue";
import LogsPanel from "@/components/LogsPanel.vue";
import FileExplorer from "@/components/FileExplorer.vue";
import EditorDock from "@/components/EditorDock.vue";
</script>

<template>
    <div class="h-screen w-screen flex flex-col bg-background">
        <!-- Top Bar -->
        <TitleBar />

        <!-- Main Content Area -->
        <div class="flex-1 overflow-hidden flex">
            <!-- Sidebar -->
            <Sidebar />

            <!-- Editor Page Content -->
            <div class="flex-1 overflow-hidden">
                <ResizablePanelGroup direction="horizontal">
                    <!-- Editor Panel -->
                    <ResizablePanel :default-size="70" :min-size="30">
                        <div class="h-full flex flex-col">
                            <ResizablePanelGroup direction="vertical">
                                <!-- Script Editor -->
                                <ResizablePanel
                                    :default-size="70"
                                    :min-size="30"
                                >
                                    <ScriptEditor />
                                </ResizablePanel>

                                <ResizableHandle />

                                <!-- Logs Panel -->
                                <ResizablePanel
                                    :default-size="30"
                                    :min-size="15"
                                >
                                    <LogsPanel />
                                </ResizablePanel>
                            </ResizablePanelGroup>

                            <!-- Dock with buttons -->
                            <EditorDock />
                        </div>
                    </ResizablePanel>

                    <ResizableHandle with-handle />

                    <!-- File Tree Panel -->
                    <ResizablePanel :default-size="25" :min-size="20">
                        <FileExplorer />
                    </ResizablePanel>
                </ResizablePanelGroup>
            </div>
        </div>
    </div>
</template>
