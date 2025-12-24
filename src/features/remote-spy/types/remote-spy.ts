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
 * Represents a unique remote (identified by numeric instance ID)
 */
export interface Remote {
  id: number; // Unique numeric ID for the remote instance
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

/**
 * Represents an attached client for remote spy
 */
export interface RemoteSpyClient {
  id: string;
  username: string;
}
