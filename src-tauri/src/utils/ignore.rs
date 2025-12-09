use std::fs;
use std::path::{Path, PathBuf};

/// Default ignore patterns for .proximaignore files
pub const DEFAULT_IGNORE_PATTERNS: &[&str] = &[
    "# Proxima Ignore File",
    "# A list of file patterns to ignored by Proxima's file tree",
    "",
    "# Version control",
    ".git",
    ".svn",
    ".hg",
    "",
    "# Python",
    "venv",
    ".venv",
    "__pycache__",
    "*.pyc",
    "*.pyo",
    "*.egg-info",
    ".pytest_cache",
    "",
    "# Node.js",
    "node_modules",
    ".npm",
    ".yarn",
    "",
    "# IDE",
    ".vscode",
    ".idea",
    "*.swp",
    "*.swo",
    "",
    "# OS",
    ".DS_Store",
    "Thumbs.db",
    "desktop.ini",
];

/// Represents a collection of ignore patterns
#[derive(Debug, Clone)]
pub struct IgnorePatterns {
    patterns: Vec<Pattern>,
}

#[derive(Debug, Clone)]
struct Pattern {
    pattern: String,
    is_negation: bool,
}

impl IgnorePatterns {
    /// Create a new IgnorePatterns from a .proximaignore file
    pub fn from_file(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::empty());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read ignore file: {}", e))?;

        Ok(Self::from_string(&content))
    }

    /// Create a new IgnorePatterns from a string
    pub fn from_string(content: &str) -> Self {
        let patterns = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| {
                if line.starts_with('!') {
                    Pattern {
                        pattern: line[1..].to_string(),
                        is_negation: true,
                    }
                } else {
                    Pattern {
                        pattern: line.to_string(),
                        is_negation: false,
                    }
                }
            })
            .collect();

        Self { patterns }
    }

    /// Create an empty IgnorePatterns
    pub fn empty() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// Check if a path should be ignored
    /// Returns true if the path matches an ignore pattern and should be excluded
    pub fn is_ignored(&self, path: &Path, is_dir: bool) -> bool {
        let path_str = path.to_string_lossy();
        let mut ignored = false;

        for pattern in &self.patterns {
            if Self::matches_pattern(&path_str, &pattern.pattern, is_dir) {
                ignored = !pattern.is_negation;
            }
        }

        ignored
    }

    /// Check if a pattern matches a path
    fn matches_pattern(path: &str, pattern: &str, is_dir: bool) -> bool {
        // Normalize paths to use forward slashes
        let path = path.replace('\\', "/");
        let pattern = pattern.replace('\\', "/");

        // Handle directory-only patterns (ending with /)
        if pattern.ends_with('/') {
            if !is_dir {
                return false;
            }
            let pattern = pattern.trim_end_matches('/');
            return Self::glob_match(&path, pattern);
        }

        // Handle patterns with path separators
        if pattern.contains('/') {
            return Self::glob_match(&path, &pattern);
        }

        // For simple patterns (no /), match against the filename or any path component
        let segments: Vec<&str> = path.split('/').collect();
        for segment in segments {
            if Self::glob_match(segment, &pattern) {
                return true;
            }
        }

        false
    }

    /// Simple glob matching supporting * and **
    fn glob_match(text: &str, pattern: &str) -> bool {
        // Handle ** for matching any number of directories
        if pattern.contains("**") {
            let parts: Vec<&str> = pattern.split("**").collect();
            if parts.len() != 2 {
                // Invalid pattern with multiple **
                return false;
            }

            let prefix = parts[0].trim_matches('/');
            let suffix = parts[1].trim_matches('/');

            let prefix_match = if prefix.is_empty() {
                true
            } else {
                text.starts_with(prefix) || Self::simple_glob_match(text, prefix)
            };

            let suffix_match = if suffix.is_empty() {
                true
            } else {
                text.ends_with(suffix) || Self::simple_glob_match(text, suffix)
            };

            return prefix_match && suffix_match;
        }

        Self::simple_glob_match(text, pattern)
    }

    /// Simple glob matching supporting * wildcard
    fn simple_glob_match(text: &str, pattern: &str) -> bool {
        let mut text_idx = 0;
        let mut pattern_idx = 0;
        let text_chars: Vec<char> = text.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();
        let mut star_idx = None;
        let mut match_idx = 0;

        while text_idx < text_chars.len() {
            if pattern_idx < pattern_chars.len() && pattern_chars[pattern_idx] == '*' {
                star_idx = Some(pattern_idx);
                match_idx = text_idx;
                pattern_idx += 1;
            } else if pattern_idx < pattern_chars.len()
                && (pattern_chars[pattern_idx] == text_chars[text_idx]
                    || pattern_chars[pattern_idx] == '?')
            {
                text_idx += 1;
                pattern_idx += 1;
            } else if let Some(star) = star_idx {
                pattern_idx = star + 1;
                match_idx += 1;
                text_idx = match_idx;
            } else {
                return false;
            }
        }

        while pattern_idx < pattern_chars.len() && pattern_chars[pattern_idx] == '*' {
            pattern_idx += 1;
        }

        pattern_idx == pattern_chars.len()
    }
}

/// Create a default .proximaignore file if it doesn't exist
pub fn ensure_ignore_file(dir: &Path) -> Result<PathBuf, String> {
    let ignore_file = dir.join(".proximaignore");

    if !ignore_file.exists() {
        let content = DEFAULT_IGNORE_PATTERNS.join("\n");
        fs::write(&ignore_file, content)
            .map_err(|e| format!("Failed to create .proximaignore: {}", e))?;
        log::info!(
            "Created default .proximaignore in {}",
            dir.display()
        );
    }

    Ok(ignore_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pattern() {
        let patterns = IgnorePatterns::from_string("node_modules\n.git");
        assert!(patterns.is_ignored(Path::new("node_modules"), true));
        assert!(patterns.is_ignored(Path::new("project/node_modules"), true));
        assert!(patterns.is_ignored(Path::new(".git"), true));
        assert!(!patterns.is_ignored(Path::new("src"), true));
    }

    #[test]
    fn test_wildcard_pattern() {
        let patterns = IgnorePatterns::from_string("*.pyc\n*.log");
        assert!(patterns.is_ignored(Path::new("test.pyc"), false));
        assert!(patterns.is_ignored(Path::new("dir/test.log"), false));
        assert!(!patterns.is_ignored(Path::new("test.py"), false));
    }

    #[test]
    fn test_directory_only_pattern() {
        let patterns = IgnorePatterns::from_string("build/");
        assert!(patterns.is_ignored(Path::new("build"), true));
        assert!(!patterns.is_ignored(Path::new("build"), false));
    }

    #[test]
    fn test_negation_pattern() {
        let patterns = IgnorePatterns::from_string("*.log\n!important.log");
        assert!(patterns.is_ignored(Path::new("test.log"), false));
        assert!(!patterns.is_ignored(Path::new("important.log"), false));
    }

    #[test]
    fn test_path_pattern() {
        let patterns = IgnorePatterns::from_string("src/temp/*");
        assert!(patterns.is_ignored(Path::new("src/temp/file.txt"), false));
        assert!(!patterns.is_ignored(Path::new("src/file.txt"), false));
    }
}
