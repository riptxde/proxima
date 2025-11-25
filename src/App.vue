<script setup lang="ts">
import { ref } from "vue";
import { Textarea } from "@/components/ui/textarea";
import {
    ResizablePanelGroup,
    ResizablePanel,
    ResizableHandle,
} from "@/components/ui/resizable";
import { Separator } from "@/components/ui/separator";
import { X, Minus, Square } from "lucide-vue-next";
import FileTreeItem from "@/components/FileTreeItem.vue";

const scriptContent = ref("");

// Mock file tree data
const fileTree = ref([
    {
        name: "Scripts",
        type: "folder",
        expanded: true,
        children: [
            { name: "example.lua", type: "file" },
            { name: "test.lua", type: "file" },
            {
                name: "Utils",
                type: "folder",
                expanded: false,
                children: [{ name: "helper.lua", type: "file" }],
            },
        ],
    },
    {
        name: "Workspace",
        type: "folder",
        expanded: true,
        children: [
            { name: "script1.lua", type: "file" },
            { name: "script2.lua", type: "file" },
        ],
    },
]);

const toggleFolder = (item: any) => {
    if (item.type === "folder") {
        item.expanded = !item.expanded;
    }
};
</script>

<template>
    <div class="h-screen w-screen flex flex-col bg-background">
        <!-- Top Bar -->
        <div
            class="h-10 bg-muted/50 border-b flex items-center justify-between px-4"
        >
            <div class="text-sm font-medium">Script Executor</div>
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
        <div class="flex-1 overflow-hidden">
            <ResizablePanelGroup direction="horizontal">
                <!-- Editor Panel -->
                <ResizablePanel :default-size="70" :min-size="30">
                    <div class="h-full flex flex-col">
                        <div class="p-4 flex-1 overflow-hidden">
                            <Textarea
                                v-model="scriptContent"
                                placeholder="-- Write your script here..."
                                class="h-full w-full resize-none font-mono text-sm"
                            />
                        </div>
                    </div>
                </ResizablePanel>

                <ResizableHandle with-handle />

                <!-- File Tree Panel -->
                <ResizablePanel :default-size="30" :min-size="20">
                    <div class="h-full flex flex-col bg-muted/20">
                        <div class="p-3 border-b">
                            <h2 class="text-sm font-semibold">File Explorer</h2>
                        </div>
                        <div class="flex-1 overflow-auto p-2">
                            <div
                                v-for="(item, index) in fileTree"
                                :key="index"
                                class="mb-1"
                            >
                                <FileTreeItem
                                    :item="item"
                                    :level="0"
                                    @toggle="toggleFolder"
                                />
                            </div>
                        </div>
                    </div>
                </ResizablePanel>
            </ResizablePanelGroup>
        </div>
    </div>
</template>

<style scoped>
/* Custom styles for file tree */
</style>
