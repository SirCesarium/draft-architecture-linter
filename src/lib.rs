//! Sweet: A blazing-fast code health and architecture analyzer.
//!
//! Provides the core logic for analyzing source code metrics,
//! managing configurations, and generating health reports.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub mod analyzer;
pub mod errors;
pub mod report;

/// Defines health metric limits.
///
/// Files exceeding these limits are flagged as "bitter".
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Thresholds {
    /// Maximum allowed source lines.
    #[serde(default = "default_max_lines")]
    pub max_lines: usize,
    /// Maximum allowed control flow nesting depth.
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
    /// Maximum allowed import statements.
    #[serde(default = "default_max_imports")]
    pub max_imports: usize,
    /// Maximum allowed repetition percentage (0-100).
    #[serde(default = "default_max_repetition")]
    pub max_repetition: f64,
}

const fn default_max_lines() -> usize {
    250
}
const fn default_max_depth() -> usize {
    5
}
const fn default_max_imports() -> usize {
    20
}
const fn default_max_repetition() -> f64 {
    10.0
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            max_lines: 250,
            max_depth: 5,
            max_imports: 20,
            max_repetition: 10.0,
        }
    }
}

/// Analyzer global configuration.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Config {
    /// Directory or file patterns excluded from analysis.
    #[serde(default = "default_excludes")]
    pub exclude: Vec<String>,
    /// Global and language-specific threshold settings.
    #[serde(default)]
    pub thresholds: ThresholdsConfig,
}

/// Threshold management including global defaults and language-specific overrides.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ThresholdsConfig {
    /// Default thresholds for all supported files.
    #[serde(default)]
    pub global: Thresholds,
    /// Overrides indexed by file extension (e.g., "rs", "java").
    #[serde(default)]
    pub overrides: HashMap<String, PartialThresholds>,
}

/// Subset of thresholds for language-specific overrides.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PartialThresholds {
    pub max_lines: Option<usize>,
    pub max_depth: Option<usize>,
    pub max_imports: Option<usize>,
    pub max_repetition: Option<f64>,
}

fn default_excludes() -> Vec<String> {
    vec![
        "node_modules".to_string(),
        "vendor".to_string(),
        "dist".to_string(),
        "target".to_string(),
        "__pycache__".to_string(),
        "build".to_string(),
        ".git".to_string(),
    ]
}

impl Config {
    /// Loads the `.swtrc` configuration from the project root.
    ///
    /// Returns the default configuration if the file is missing or malformed.
    #[must_use]
    pub fn load(root: &Path) -> Self {
        let config_path = root.join(".swtrc");
        fs::read_to_string(config_path).map_or_else(
            |_| Self {
                exclude: vec![],
                thresholds: ThresholdsConfig::default(),
            },
            |content| serde_json::from_str(&content).unwrap_or_default(),
        )
    }

    /// Resolves effective thresholds for a given file extension.
    ///
    /// Combines global defaults with language-specific overrides.
    #[must_use]
    pub fn get_thresholds(&self, extension: &str) -> Thresholds {
        let mut t = self.thresholds.global.clone();
        if let Some(over) = self.thresholds.overrides.get(extension) {
            if let Some(v) = over.max_lines {
                t.max_lines = v;
            }
            if let Some(v) = over.max_depth {
                t.max_depth = v;
            }
            if let Some(v) = over.max_imports {
                t.max_imports = v;
            }
            if let Some(v) = over.max_repetition {
                t.max_repetition = v;
            }
        }
        t
    }

    /// Checks if a file is supported based on its extension.
    ///
    /// Validates extension support and ensures the path is a file if it exists.
    #[must_use]
    pub fn is_supported_file(path: &Path) -> bool {
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let supported = matches!(extension, "rs" | "ts" | "js" | "java" | "cs" | "py");

        if path.exists() {
            path.is_file() && supported
        } else {
            supported
        }
    }
}

/// Results of a single file analysis.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileReport {
    /// Path to the analyzed file.
    pub path: PathBuf,
    /// Total source lines.
    pub lines: usize,
    /// Total import statements.
    pub imports: usize,
    /// Maximum nesting depth detected.
    pub max_depth: usize,
    /// Repetition percentage (0.0 to 100.0).
    pub repetition: f64,
    /// True if all metrics are within thresholds.
    pub is_sweet: bool,
    /// List of threshold violations.
    pub issues: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_overrides() {
        let mut config = Config::default();
        config.thresholds.overrides.insert(
            "java".to_string(),
            PartialThresholds {
                max_imports: Some(100),
                ..Default::default()
            },
        );

        let t = config.get_thresholds("java");
        assert_eq!(t.max_imports, 100);
        assert_eq!(t.max_lines, 250);
    }

    #[test]
    fn test_is_supported_file() {
        assert!(Config::is_supported_file(Path::new("test.rs")));
        assert!(!Config::is_supported_file(Path::new("test.txt")));
    }

    #[cfg(feature = "schema")]
    #[test]
    fn generate_schema() {
        use schemars::schema_for;
        let schema = schema_for!(Config);
        let schema_json = serde_json::to_string_pretty(&schema).unwrap();
        fs::write("schema.json", schema_json).unwrap();
    }
}
