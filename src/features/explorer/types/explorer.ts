export interface ExplorerItem {
  id: string;
  name: string;
  className: string;
  hasChildren: boolean;
  children: ExplorerItem[];
}

export interface ExplorerProperty {
  name: string;
  type: string;
  value: string;
  readOnly: boolean;
  deprecated: boolean;
  hidden: boolean;
  notScriptable: boolean;
}

export interface ExplorerClient {
  id: string;
  username: string;
}

export interface ExplorerSearchResult {
  id: string;
  name: string;
  className: string;
  path: number[]; // Array of ancestor IDs forming path to instance
  pathString: string; // Human-readable path string (e.g., "Workspace > Model > Part")
}
