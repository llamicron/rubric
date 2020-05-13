//! File system helpers

// std uses
use std::fs;
use std::path::{Path, PathBuf};


/// Returns true if a file or dir at the given path exists
///
/// ```rust
/// use lab_grader::helpers::fs;
///
/// assert!(fs::file_exists("Cargo.toml"));
/// ```
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    PathBuf::from(path.as_ref()).exists()
}

/// Returns true if a file contains the provided string
///
/// **Warning:** if the file couldn't be read from
/// (doesn't exist, invalid permission, etc), this function will
/// return `false`, not panic!
///
/// ```rust
/// use lab_grader::helpers::fs;
///
/// assert!(fs::file_contains("Cargo.toml", "version"));
/// ```
pub fn file_contains<P: AsRef<Path>>(path: P, needle: &str) -> bool {
    if let Ok(content) = fs::read_to_string(path) {
        return content.contains(needle)
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_exists() {
        assert!(file_exists("test_data/test_batch.yml"));
        assert!(!file_exists(""));
        assert!(!file_exists("doesntexist"));
        assert!(file_exists("src"));
        assert!(file_exists("src/"));
    }

    #[test]
    fn test_file_contains() {
        assert!(file_contains("test_data/test_batch.yml", "criteria"));
        assert!(!file_contains("test_data/test_batch.yml", "something it doesn't contain"));
        assert!(!file_contains("src/", "doesn't matter"));
    }
}
