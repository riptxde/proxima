<script setup lang="ts">
import { ref } from "vue";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "@/components/ui/dialog";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

defineProps<{
    open: boolean;
}>();

const emit = defineEmits<{
    "update:open": [value: boolean];
    save: [filename: string, folder: "Scripts" | "AutoExec"];
}>();

const filename = ref("");
const selectedFolder = ref<"Scripts" | "AutoExec">("Scripts");

const handleSave = () => {
    if (filename.value.trim()) {
        emit("save", filename.value.trim(), selectedFolder.value);
        // Reset form
        filename.value = "";
        selectedFolder.value = "Scripts";
        emit("update:open", false);
    }
};

const handleCancel = () => {
    // Reset form
    filename.value = "";
    selectedFolder.value = "Scripts";
    emit("update:open", false);
};
</script>

<template>
    <Dialog :open="open" @update:open="(val) => emit('update:open', val)">
        <DialogContent class="sm:max-w-[425px]">
            <DialogHeader>
                <DialogTitle>Save Script</DialogTitle>
                <DialogDescription>
                    Enter a file name and select the destination folder for your
                    script.
                </DialogDescription>
            </DialogHeader>
            <div class="grid gap-4 py-4">
                <div class="grid gap-2">
                    <label for="filename" class="text-sm font-medium">
                        File Name
                    </label>
                    <Input
                        id="filename"
                        v-model="filename"
                        placeholder="my_script.luau"
                        @keydown.enter="handleSave"
                    />
                </div>
                <div class="grid gap-2">
                    <label for="folder" class="text-sm font-medium">
                        Destination
                    </label>
                    <Select v-model="selectedFolder">
                        <SelectTrigger id="folder">
                            <SelectValue placeholder="Select folder" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="Scripts"> Scripts </SelectItem>
                            <SelectItem value="AutoExec"> AutoExec </SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </div>
            <DialogFooter>
                <Button variant="outline" @click="handleCancel">
                    Cancel
                </Button>
                <Button @click="handleSave" :disabled="!filename.trim()">
                    Save
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>
