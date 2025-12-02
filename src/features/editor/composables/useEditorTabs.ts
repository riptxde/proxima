import { ref } from "vue";
import type { Tab } from "../types/tab";

const tabs = ref<Tab[]>([
  { id: 1, name: "Script 1", content: "-- Write your script here..." },
]);

const activeTabId = ref(1);
const nextTabId = ref(2);

export function useEditorTabs() {
  const openFile = (fileName: string, content: string, filePath: string) => {
    // Normalize path to use forward slashes
    const normalizedPath = filePath.replace(/\\/g, "/");

    // Check if file is already open
    const existingTab = tabs.value.find(
      (tab) => tab.filePath === normalizedPath,
    );

    if (existingTab) {
      // Focus existing tab
      activeTabId.value = existingTab.id;
      return;
    }

    // Create new tab for the file
    const newTab: Tab = {
      id: nextTabId.value,
      name: fileName,
      content: content,
      filePath: normalizedPath,
    };

    nextTabId.value++;
    tabs.value.push(newTab);
    activeTabId.value = newTab.id;
  };

  const addTab = () => {
    // Find all "Script %d" numbers in existing tabs
    const scriptPattern = /^Script (\d+)$/;
    const takenNumbers = new Set<number>();

    tabs.value.forEach((tab) => {
      const match = tab.name.match(scriptPattern);
      if (match && match[1]) {
        const num = parseInt(match[1], 10);
        takenNumbers.add(num);
      }
    });

    // Find the earliest available number starting from 1
    let scriptNumber = 1;
    while (takenNumbers.has(scriptNumber)) {
      scriptNumber++;
    }

    const newTab: Tab = {
      id: nextTabId.value,
      name: `Script ${scriptNumber}`,
      content: "-- Write your script here...",
    };
    nextTabId.value++;
    tabs.value.push(newTab);
    activeTabId.value = newTab.id;
  };

  const closeTab = (tabId: number) => {
    if (tabs.value.length === 1) return;

    const index = tabs.value.findIndex((tab) => tab.id === tabId);
    if (index === -1) return;

    tabs.value.splice(index, 1);

    if (activeTabId.value === tabId) {
      const newIndex = Math.max(0, index - 1);
      const newTab = tabs.value[newIndex];
      if (newTab) {
        activeTabId.value = newTab.id;
      }
    }
  };

  const selectTab = (tabId: number) => {
    activeTabId.value = tabId;
  };

  const renameTab = (tabId: number, newName: string) => {
    const tab = tabs.value.find((t) => t.id === tabId);
    if (tab && newName.trim()) {
      tab.name = newName.trim();
    }
  };

  const updateTabContent = (tabId: number, content: string) => {
    const tab = tabs.value.find((t) => t.id === tabId);
    if (tab) {
      tab.content = content;
    }
  };

  const clearActiveTab = () => {
    const tab = tabs.value.find((t) => t.id === activeTabId.value);
    if (tab) {
      tab.content = "";
    }
  };

  const openFileAsTab = (fileName: string, content: string) => {
    // Create new tab without filePath (no duplicate detection)
    const newTab: Tab = {
      id: nextTabId.value,
      name: fileName,
      content: content,
    };

    nextTabId.value++;
    tabs.value.push(newTab);
    activeTabId.value = newTab.id;
  };

  const getActiveTabContent = () => {
    const tab = tabs.value.find((t) => t.id === activeTabId.value);
    return tab?.content ?? "";
  };

  const getActiveTabFilePath = () => {
    const tab = tabs.value.find((t) => t.id === activeTabId.value);
    return tab?.filePath;
  };

  const updateActiveTabFilePath = (filePath: string) => {
    const tab = tabs.value.find((t) => t.id === activeTabId.value);
    if (tab) {
      // Normalize path to use forward slashes
      tab.filePath = filePath.replace(/\\/g, "/");
    }
  };

  return {
    tabs,
    activeTabId,
    openFile,
    addTab,
    closeTab,
    selectTab,
    renameTab,
    updateTabContent,
    clearActiveTab,
    openFileAsTab,
    getActiveTabContent,
    getActiveTabFilePath,
    updateActiveTabFilePath,
  };
}
