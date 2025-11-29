export interface FileNodeFile {
  type: "file";
  id: string;
  name: string;
}

export interface FileNodeFolder {
  type: "folder";
  id: string;
  name: string;
  children: FileNode[];
}

export type FileNode = FileNodeFile | FileNodeFolder;
