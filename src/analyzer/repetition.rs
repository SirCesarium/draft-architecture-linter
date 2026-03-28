//! Duplicate code detection using sliding window hashing.

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Measures the percentage of repetitive code blocks in the content.
///
/// Uses normalized line hashing and a sliding window to identify duplicate code chunks.
#[must_use]
pub fn analyze_repetition(content: &str) -> f64 {
    let lines_content: Vec<&str> = content
        .lines()
        .filter(|l| !l.contains("@sweetignore"))
        .collect();

    if lines_content.is_empty() {
        return 0.0;
    }

    let hashes: Vec<u64> = lines_content
        .iter()
        .map(|&l| {
            let mut s = DefaultHasher::new();
            for c in l.chars().filter(|c| !c.is_whitespace()) {
                for lc in c.to_lowercase() {
                    lc.hash(&mut s);
                }
            }
            s.finish()
        })
        .collect();

    let window_size = 4;
    if hashes.len() < window_size {
        return 0.0;
    }

    let mut repetitive_lines = vec![false; hashes.len()];
    let mut chunks: HashMap<&[u64], Vec<usize>> = HashMap::with_capacity(hashes.len());

    for i in 0..=hashes.len() - window_size {
        let chunk = &hashes[i..i + window_size];

        if lines_content[i..i + window_size]
            .iter()
            .all(|l| l.trim().len() < 3)
        {
            continue;
        }

        chunks.entry(chunk).or_default().push(i);
    }

    for occurrences in chunks.values().filter(|v| v.len() > 1) {
        for &start_idx in occurrences {
            for r in &mut repetitive_lines[start_idx..start_idx + window_size] {
                *r = true;
            }
        }
    }

    let repeated_count = repetitive_lines.iter().filter(|&&r| r).count();
    #[allow(clippy::cast_precision_loss)]
    {
        (repeated_count as f64 / hashes.len() as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repetition() {
        let no_rep =
            "fn main() {\n    let x = 1;\n    let y = 2;\n    let z = 3;\n    let w = 4;\n}";
        assert!(analyze_repetition(no_rep).abs() < f64::EPSILON);

        let rep = "let a = 1;\nlet b = 2;\nlet c = 3;\nlet d = 4;\nlet a = 1;\nlet b = 2;\nlet c = 3;\nlet d = 4;";
        assert!(analyze_repetition(rep) > 0.0);

        let fuzzy = "let a = 1;\nlet b = 2;\nlet c = 3;\nlet d = 4;\nlet a=1;\nlet b=2;\nlet c=3;\nlet d=4;";
        assert!(analyze_repetition(fuzzy) > 0.0);
    }
}
