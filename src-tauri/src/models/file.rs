use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum FileNode {
    #[serde(rename = "file")]
    File {
        id: String,
        name: String,
        path: String, // Relative path from base directory
    },
    #[serde(rename = "folder")]
    Folder {
        id: String,
        name: String,
        children: Vec<FileNode>,
    },
}
