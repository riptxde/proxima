<script setup lang="ts">
import { Tree, Folder, File } from "@/components/ui/file-tree";
import { Card } from "@/components/ui/card";
</script>

<template>
    <Card class="h-full overflow-auto p-2">
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
    </Card>
</template>
