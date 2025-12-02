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

export function levelFromNumber(level: number): LogLevel | null {
  switch (level) {
    case 0:
      return "info";
    case 1:
      return "success";
    case 2:
      return "warning";
    case 3:
      return "error";
    default:
      return null;
  }
}
