/**
 * Represents a connected WebSocket client
 * Shared across all features (executor, explorer, remote spy)
 */
export interface Client {
  id: string;
  username: string;
}
