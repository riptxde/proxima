import { ref, computed } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import type { Log, LogLevel, LogFilters } from "../types/log";
import { levelFromNumber } from "../types/log";

const logs = ref<Log[]>([]);
const filters = ref<LogFilters>({
  levels: ["info", "warning", "error", "success"],
  search: "",
});

let unlistenFn: UnlistenFn | null = null;

export function useLogs() {
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

  const addLog = (level: LogLevel | number, message: string) => {
    let logLevel: LogLevel;

    // Convert numeric level to string level
    if (typeof level === "number") {
      const converted = levelFromNumber(level);
      if (!converted) {
        // Log invalid level error
        invoke("add_log", {
          level: 3,
          message: `Invalid log level received: ${level}`,
        }).catch(() => {
          // Can't log the error about logging, silently fail
        });
        return;
      }
      logLevel = converted;
    } else {
      logLevel = level;
    }

    logs.value.unshift({
      id: Date.now().toString(),
      timestamp: new Date(),
      level: logLevel,
      message,
    });
  };

  const initializeLogListener = async () => {
    if (unlistenFn) return;

    unlistenFn = await listen<{ level: number; message: string }>(
      "log-message",
      (event) => {
        const { level, message } = event.payload;
        addLog(level, message);
      },
    );
  };

  const cleanup = () => {
    if (unlistenFn) {
      unlistenFn();
      unlistenFn = null;
    }
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
    initializeLogListener,
    cleanup,
    clearLogs,
    toggleLevelFilter,
    setSearchFilter,
    getLevelCount,
  };
}
