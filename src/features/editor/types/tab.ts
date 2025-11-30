export interface Tab {
  id: number;
  name: string;
  content: string;
  filePath?: string;
}

export interface TabUIState {
  editingTabId: number | null;
  editingTabName: string;
  inputWidth: number;
}
