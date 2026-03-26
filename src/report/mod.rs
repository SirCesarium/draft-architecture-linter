//! Logic for generating health reports in various formats.

pub mod json;
pub mod terminal;

use crate::FileReport;
use std::path::Path;

/// High-level function to handle report generation.
///
/// Dispatches reports to the terminal and/or a JSON file based on configuration.
pub fn print_reports(reports: &[FileReport], quiet: bool, json_path: Option<&Path>) {
    if let Some(path) = json_path {
        json::write_json_report(reports, path);
    }

    // Always output a terminal summary unless in JSON-only stdout mode.
    terminal::print_summary(reports, quiet);
}
