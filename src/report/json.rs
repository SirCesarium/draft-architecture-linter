//! Logic for exporting reports to JSON.

use crate::FileReport;
use std::fs;
use std::path::Path;

/// Writes the provided reports to a file in JSON format.
pub fn write_json_report(reports: &[FileReport], path: &Path) {
    if let Ok(json) = serde_json::to_string_pretty(reports) {
        let _ = fs::write(path, json);
    }
}
