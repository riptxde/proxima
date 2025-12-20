export type RemoteDirection = "outgoing" | "incoming";
export type RemoteType = "RemoteEvent" | "RemoteFunction";

/**
 * Represents a single remote call event
 */
export interface RemoteCall {
  id: string;
  timestamp: Date;
  direction: RemoteDirection;
  arguments: RemoteArgument[];
  returnValue?: RemoteArgument;
  callingScript?: string;
  callingScriptPath?: string;
}

/**
 * Represents an argument or return value
 */
export interface RemoteArgument {
  type: string;
  value: string;
}

/**
 * Represents a unique remote (identified by name + path + type)
 */
export interface Remote {
  id: string; // Unique identifier: `${name}|${path}|${type}`
  name: string;
  path: string;
  type: RemoteType;
  calls: RemoteCall[];
}

/**
 * Filter state for remote spy
 */
export interface RemoteSpyFilters {
  directions: RemoteDirection[];
  types: RemoteType[];
  search: string;
}
