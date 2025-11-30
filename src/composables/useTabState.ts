import { ref } from "vue";

export interface Tab {
  id: number;
  name: string;
  content: string;
  filePath?: string; // Optional path to the actual file
}

const tabs = ref<Tab[]>([
  { id: 1, name: "Script 1", content: "-- Write your script here..." },
]);

const activeTabId = ref(1);
const nextTabId = ref(2);

export function useTabState() {
  const openFile = (fileName: string, content: string, filePath: string) => {
    // Check if file is already open
    const existingTab = tabs.value.find((tab) => tab.filePath === filePath);

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
      filePath: filePath,
    };

    nextTabId.value++;
    tabs.value.push(newTab);
    activeTabId.value = newTab.id;
  };

  const addTab = () => {
    const newTab: Tab = {
      id: nextTabId.value,
      name: `Script ${nextTabId.value}`,
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
  };
}
