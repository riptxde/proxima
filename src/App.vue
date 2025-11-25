<script setup lang="ts">
import { ref } from "vue";
import { Textarea } from "@/components/ui/textarea";
import {
    ResizablePanelGroup,
    ResizablePanel,
    ResizableHandle,
} from "@/components/ui/resizable";
import { Button } from "@/components/ui/button";
import { X, Minus, Square, CodeXml, Library, Settings } from "lucide-vue-next";
import { Tree, Folder, File } from "@/components/ui/file-tree";

const scriptContent = ref("");

// Mock logs data
const logs = ref(["Welcome to Riptide", "Ready to execute scripts..."]);
</script>

<template>
    <div class="h-screen w-screen flex flex-col bg-background">
        <!-- Top Bar -->
        <div
            class="h-10 bg-muted/50 border-b flex items-center justify-between px-4"
        >
            <div class="text-sm font-medium">Riptide</div>
            <div class="flex items-center gap-2">
                <button
                    class="h-6 w-6 flex items-center justify-center hover:bg-muted rounded transition-colors"
                >
                    <Minus :size="16" />
                </button>
                <button
                    class="h-6 w-6 flex items-center justify-center hover:bg-muted rounded transition-colors"
                >
                    <Square :size="14" />
                </button>
                <button
                    class="h-6 w-6 flex items-center justify-center hover:bg-destructive hover:text-destructive-foreground rounded transition-colors"
                >
                    <X :size="16" />
                </button>
            </div>
        </div>

        <!-- Main Content Area -->
        <div class="flex-1 overflow-hidden flex">
            <!-- Sidebar -->
            <div
                class="w-16 bg-sidebar border-r border-sidebar-border flex flex-col items-center py-4 gap-2"
            >
                <button
                    class="h-12 w-12 flex items-center justify-center rounded-lg bg-sidebar-accent text-sidebar-accent-foreground transition-colors"
                    title="Editor"
                >
                    <CodeXml :size="20" />
                </button>
                <button
                    class="h-12 w-12 flex items-center justify-center rounded-lg hover:bg-sidebar-accent hover:text-sidebar-accent-foreground text-sidebar-foreground/60 transition-colors"
                    title="Script Hub"
                >
                    <Library :size="20" />
                </button>
                <button
                    class="h-12 w-12 flex items-center justify-center rounded-lg hover:bg-sidebar-accent hover:text-sidebar-accent-foreground text-sidebar-foreground/60 transition-colors"
                    title="Settings"
                >
                    <Settings :size="20" />
                </button>
            </div>

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
                                    <div class="h-full overflow-hidden">
                                        <Textarea
                                            v-model="scriptContent"
                                            placeholder="-- Write your script here..."
                                            class="h-full w-full resize-none font-mono text-sm rounded-none border-0 px-4 py-3 shadow-none focus-visible:ring-0"
                                        />
                                    </div>
                                </ResizablePanel>

                                <ResizableHandle />

                                <!-- Logs Panel -->
                                <ResizablePanel
                                    :default-size="30"
                                    :min-size="15"
                                >
                                    <div
                                        class="h-full flex flex-col bg-muted/20"
                                    >
                                        <div class="px-4 py-2 border-b">
                                            <h3 class="text-sm font-semibold">
                                                Logs
                                            </h3>
                                        </div>
                                        <div
                                            class="flex-1 overflow-auto p-4 font-mono text-sm"
                                        >
                                            <div
                                                v-for="(log, index) in logs"
                                                :key="index"
                                                class="text-muted-foreground mb-1"
                                            >
                                                {{ log }}
                                            </div>
                                        </div>
                                    </div>
                                </ResizablePanel>
                            </ResizablePanelGroup>

                            <!-- Dock with buttons -->
                            <div
                                class="border-t bg-muted/30 p-2 flex items-center gap-2"
                            >
                                <Button variant="default" size="sm"
                                    >Execute</Button
                                >
                                <Button variant="outline" size="sm"
                                    >Clear</Button
                                >
                                <Button variant="outline" size="sm"
                                    >Open</Button
                                >
                                <Button variant="outline" size="sm"
                                    >Save</Button
                                >
                            </div>
                        </div>
                    </ResizablePanel>

                    <ResizableHandle with-handle />

                    <!-- File Tree Panel -->
                    <ResizablePanel :default-size="30" :min-size="20">
                        <div class="h-full flex flex-col bg-muted/20">
                            <div class="p-3 border-b">
                                <h2 class="text-sm font-semibold">
                                    File Explorer
                                </h2>
                            </div>
                            <div class="flex-1 overflow-auto p-2">
                                <Tree
                                    :initial-selected-id="'scripts'"
                                    :initial-expanded-items="[
                                        'scripts',
                                        'autoexec',
                                    ]"
                                    :elements="fileTreeElements"
                                >
                                    <Folder id="scripts" name="Scripts">
                                        <File id="1" name="example.lua" />
                                        <File id="2" name="test.lua" />
                                        <Folder id="3" name="Utils">
                                            <File id="4" name="helper.lua" />
                                        </Folder>
                                    </Folder>
                                    <Folder id="autoexec" name="AutoExec">
                                        <File id="5" name="script1.lua" />
                                        <File id="6" name="script2.lua" />
                                    </Folder>
                                </Tree>
                            </div>
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* Custom styles for file tree */
</style>
