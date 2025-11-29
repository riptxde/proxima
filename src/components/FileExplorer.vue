<script setup lang="ts">
import { Tree, Folder, File } from "@/components/ui/file-tree";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Search } from "lucide-vue-next";
</script>

<template>
    <Card class="h-full p-2 flex flex-col gap-3">
        <!-- Search Bar -->
        <div class="relative">
            <Search
                class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground"
            />
            <Input
                type="text"
                placeholder="Search"
                class="pl-9 border-none h-[46px] text-sm bg-container-bg"
            />
        </div>

        <!-- File Tree -->
        <div class="flex-1 overflow-auto">
            <Tree
                :initial-selected-id="'scripts'"
                :initial-expanded-items="['scripts', 'autoexec']"
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
    </Card>
</template>
