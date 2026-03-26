//! Logic for measuring file volume (`SLoC`).

/// Counts the number of lines in a string content.
#[must_use]
pub fn count_lines(content: &str) -> usize {
    content.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines() {
        let code = "line1\nline2\nline3";
        assert_eq!(count_lines(code), 3);
    }

    #[test]
    fn test_count_empty() {
        assert_eq!(count_lines(""), 0);
    }
}
