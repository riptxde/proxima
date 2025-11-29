export interface FileNodeFile {
  type: "file";
  id: string;
  name: string;
  path: string; // Relative path from base directory
}

export interface FileNodeFolder {
  type: "folder";
  id: string;
  name: string;
  children: FileNode[];
}

export type FileNode = FileNodeFile | FileNodeFolder;
