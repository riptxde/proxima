import { ref, computed } from "vue";
import type { Log, LogLevel, LogFilters } from "../types/log";

const logs = ref<Log[]>([]);
const filters = ref<LogFilters>({
  levels: ["info", "warning", "error", "success"],
  search: "",
});

// Generate dummy logs for testing
const generateDummyLogs = (): Log[] => {
  const dummyLogs: Log[] = [
    {
      id: "1",
      timestamp: new Date(Date.now() - 1000 * 60 * 5), // 5 minutes ago
      level: "success",
      message: "Script executed successfully",
    },
    {
      id: "2",
      timestamp: new Date(Date.now() - 1000 * 60 * 10), // 10 minutes ago
      level: "info",
      message: "Client 'Player123' connected to WebSocket server",
    },
    {
      id: "3",
      timestamp: new Date(Date.now() - 1000 * 60 * 15), // 15 minutes ago
      level: "warning",
      message: "Script execution took longer than expected (2.5s)",
    },
    {
      id: "4",
      timestamp: new Date(Date.now() - 1000 * 60 * 20), // 20 minutes ago
      level: "info",
      message: "File saved: Scripts/test.lua",
    },
    {
      id: "5",
      timestamp: new Date(Date.now() - 1000 * 60 * 25), // 25 minutes ago
      level: "error",
      message: "Failed to read file: invalid path",
    },
    {
      id: "6",
      timestamp: new Date(Date.now() - 1000 * 60 * 30), // 30 minutes ago
      level: "success",
      message: "Directories initialized successfully",
    },
    {
      id: "7",
      timestamp: new Date(Date.now() - 1000 * 60 * 35), // 35 minutes ago
      level: "warning",
      message: "Client disconnected unexpectedly",
    },
    {
      id: "8",
      timestamp: new Date(Date.now() - 1000 * 60 * 40), // 40 minutes ago
      level: "info",
      message: "Monaco editor initialized",
    },
    {
      id: "9",
      timestamp: new Date(Date.now() - 1000 * 60 * 45), // 45 minutes ago
      level: "error",
      message: "WebSocket server failed to start on port 13376",
    },
    {
      id: "10",
      timestamp: new Date(Date.now() - 1000 * 60 * 50), // 50 minutes ago
      level: "success",
      message: "Application started successfully",
    },
    {
      id: "11",
      timestamp: new Date(Date.now() - 1000 * 60 * 55), // 55 minutes ago
      level: "info",
      message: "Loading configuration from disk",
    },
    {
      id: "12",
      timestamp: new Date(Date.now() - 1000 * 60 * 60), // 1 hour ago
      level: "warning",
      message: "Deprecated API usage detected in script",
    },
  ];

  return dummyLogs.sort(
    (a, b) => b.timestamp.getTime() - a.timestamp.getTime(),
  );
};

export function useLogs() {
  // Initialize with dummy logs
  if (logs.value.length === 0) {
    logs.value = generateDummyLogs();
  }

  const filteredLogs = computed(() => {
    return logs.value.filter((log) => {
      // Filter by level
      if (!filters.value.levels.includes(log.level)) {
        return false;
      }

      // Filter by search
      if (filters.value.search) {
        const searchLower = filters.value.search.toLowerCase();
        return log.message.toLowerCase().includes(searchLower);
      }

      return true;
    });
  });

  const addLog = (level: LogLevel, message: string) => {
    logs.value.unshift({
      id: Date.now().toString(),
      timestamp: new Date(),
      level,
      message,
    });
  };

  const clearLogs = () => {
    logs.value = [];
  };

  const toggleLevelFilter = (level: LogLevel) => {
    const index = filters.value.levels.indexOf(level);
    if (index > -1) {
      filters.value.levels.splice(index, 1);
    } else {
      filters.value.levels.push(level);
    }
  };

  const setSearchFilter = (search: string) => {
    filters.value.search = search;
  };

  const getLevelCount = (level: LogLevel) => {
    return logs.value.filter((log) => log.level === level).length;
  };

  return {
    logs: filteredLogs,
    allLogs: logs,
    filters,
    addLog,
    clearLogs,
    toggleLevelFilter,
    setSearchFilter,
    getLevelCount,
  };
}
