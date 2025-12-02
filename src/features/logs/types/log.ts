export type LogLevel = "info" | "warning" | "error" | "success";

export interface Log {
  id: string;
  timestamp: Date;
  level: LogLevel;
  message: string;
}

export interface LogFilters {
  levels: LogLevel[];
  search: string;
}
