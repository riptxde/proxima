mod file_tree;
mod files;

pub use file_tree::read_file_tree;
pub use files::{get_scripts_path, initialize_directories, read_file_content, save_file};
