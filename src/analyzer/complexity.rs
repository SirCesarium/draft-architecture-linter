//! Logic for measuring code complexity and control flow depth.

/// Estimates the maximum nesting depth of a file's control flow.
///
/// Analysis is based on leading whitespace indentation.
/// Each tab or `indent_size` spaces equals one nesting level.
#[must_use]
pub fn analyze_depth(content: &str, indent_size: usize) -> usize {
    let mut max_depth = 0;

    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }

        let leading_whitespace = line.len() - trimmed.len();

        let depth = if line.starts_with('\t') {
            leading_whitespace
        } else {
            leading_whitespace / indent_size
        };

        if depth > max_depth {
            max_depth = depth;
        }
    }

    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_spaces() {
        let code = "fn main() {\n    if true {\n        println!();\n    }\n}";
        assert_eq!(analyze_depth(code, 4), 2);
    }

    #[test]
    fn test_depth_tabs() {
        let code = "fn main() {\n\tif true {\n\t\tprintln!();\n\t}\n}";
        assert_eq!(analyze_depth(code, 4), 2);
    }
}
