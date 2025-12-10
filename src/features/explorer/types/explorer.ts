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
