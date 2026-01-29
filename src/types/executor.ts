/**
 * Request payload for executing scripts on clients
 * Used by both direct executor and HTTP executor
 */
export interface ExecuteRequest {
  client_ids: string[];
  script: string;
  redirect: boolean;
}
