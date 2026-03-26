//! Logic for measuring file volume (`SLoC`).

/// Counts the number of lines in a string content.
#[must_use]
pub fn count_lines(content: &str) -> usize {
    content.lines().count()
}
