export interface Client {
  id: string;
  username: string;
}

export interface ExecuteRequest {
  client_ids: string[];
  script: string;
}
