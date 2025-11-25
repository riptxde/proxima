<script setup lang="ts">
import { Tree, Folder, File } from "@/components/ui/file-tree";
</script>

<template>
    <div class="h-full flex flex-col bg-muted/20">
        <div class="p-3 border-b">
            <h2 class="text-sm font-semibold">File Explorer</h2>
        </div>
        <div class="flex-1 overflow-auto p-2">
            <Tree
                :initial-selected-id="'scripts'"
                :initial-expanded-items="['scripts', 'autoexec']"
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
</template>
