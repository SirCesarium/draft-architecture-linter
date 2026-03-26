//! Logic for analyzing control flow depth through indentation.

/// Estimates the maximum nesting depth of a file's control flow.
///
/// It uses a heuristic based on leading whitespace.
/// Assuming 4 spaces or 1 tab equals one nesting level.
#[must_use]
pub fn analyze_depth(content: &str) -> usize {
    let mut max_depth = 0;

    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }

        let leading_whitespace = line.len() - trimmed.len();

        // 4 spaces = 1 level, 1 tab = 1 level.
        let depth = if line.starts_with('\t') {
            leading_whitespace
        } else {
            leading_whitespace / 4
        };

        if depth > max_depth {
            max_depth = depth;
        }
    }

    max_depth
}
