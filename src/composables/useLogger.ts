import { invoke } from "@tauri-apps/api/core";
import type { LogLevel } from "@/features/logs/types/log";

/**
 * Universal logging composable for the application.
 * Provides a consistent interface for logging to the backend.
 */
export function useLogger() {
  /**
   * Add a log entry to the backend logging system.
   * The backend will emit a log-message event that the frontend listens to.
   *
   * @param level - Log level: "info" | "success" | "warning" | "error"
   * @param message - Log message to display
   */
  const addLog = (level: LogLevel, message: string): void => {
    // Convert log level string to number for backend
    const levelMap: Record<LogLevel, number> = {
      info: 0,
      success: 1,
      warning: 2,
      error: 3,
    };

    const levelNumber = levelMap[level];

    // Invoke backend command to add log
    invoke("add_log", { level: levelNumber, message }).catch((error) => {
      console.error(`Failed to add log [${level}]: ${message}`, error);
    });
  };

  return {
    addLog,
  };
}
