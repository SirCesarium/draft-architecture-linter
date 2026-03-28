//! Analysis exclusion logic based on @sweetignore comments.

/// Checks if a file should be ignored globally via a top-level @sweetignore.
#[must_use]
pub fn is_file_ignored(content: &str) -> bool {
    content
        .lines()
        .take(10)
        .any(|line| line.contains("@sweetignore"))
}

/// Checks if a specific line or block is marked for exclusion.
#[must_use]
pub fn is_line_ignored(line: &str) -> bool {
    line.contains("@sweetignore")
}
