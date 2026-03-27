//! Main entry point for the Sweet CLI.
//!
//! Handles CLI argument parsing and orchestrates the `AnalysisEngine` and Reporting.

use clap::Parser;
use console::style;
use std::fs;
use std::path::{Path, PathBuf};
use swt::Config;
use swt::analyzer::AnalysisEngine;

const ASCII: &str = r"
                            __ 
   ______      _____  ___  / /_
  / ___/ | /| / / _ \/ _ \/ __/
 /__  /| |/ |/ /  __/  __/ /_  
/____/ |__/|__/\___/\___/\__/  ";

/// CLI Arguments for Sweet.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to analyze.
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output report in JSON format (optional: write to file).
    #[allow(clippy::option_option)]
    #[arg(long, value_name = "FILE")]
    json: Option<Option<PathBuf>>,

    /// Disable ASCII art and decorations (CI mode).
    #[arg(short, long)]
    quiet: bool,

    /// Remove comments from a specific file.
    #[arg(long, value_name = "FILE")]
    uncomment: Option<PathBuf>,

    /// Remove even doc comments (///, /**) when using --uncomment.
    #[arg(long)]
    aggressive: bool,
}

fn main() -> std::process::ExitCode {
    let args = Args::parse();

    // Handle specialized tool: Uncommenting.
    if let Some(file_path) = args.uncomment {
        if handle_uncomment(&file_path, args.aggressive) {
            return std::process::ExitCode::SUCCESS;
        }
        return std::process::ExitCode::FAILURE;
    }

    // Initialize configuration and analysis engine.
    let config = Config::load(&args.path);
    let engine = AnalysisEngine::new(args.path.clone(), config);

    // Provide visual feedback if appropriate.
    if !args.quiet && args.json.is_none() {
        show_branding();
    }

    // Execute core logic.
    let reports = engine.run(args.quiet, args.json.is_none());

    if reports.is_empty() {
        if !args.quiet {
            println!("\n{}", style(" 📭 No supported files found to analyze.").yellow().bold());
        }
        return std::process::ExitCode::SUCCESS;
    }

    let bitter_count = reports.iter().filter(|r| !r.is_sweet).count();

    // Dispatch results to the requested reporting channel.
    if let Some(json_opt) = &args.json {
        handle_json_reporting(&reports, json_opt.as_ref());
    } else {
        swt::report::print_reports(&reports, args.quiet, None);
    }

    if bitter_count > 0 {
        std::process::ExitCode::FAILURE
    } else {
        std::process::ExitCode::SUCCESS
    }
}

/// Dispatches JSON reports to a file or stdout.
fn handle_json_reporting(reports: &[swt::FileReport], json_opt: Option<&PathBuf>) {
    if let Some(path) = json_opt {
        swt::report::json::write_json_report(reports, path);
    } else if let Ok(json) = serde_json::to_string_pretty(&reports) {
        println!("{json}");
    }
}

/// Handles the 'uncomment' feature by stripping comments and rewriting the file.
fn handle_uncomment(path: &Path, aggressive: bool) -> bool {
    match fs::read_to_string(path) {
        Ok(content) => {
            let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let clean = swt::analyzer::uncomment::remove_comments(&content, extension, aggressive);

            if fs::write(path, clean).is_ok() {
                println!("{}", style("Uncommented!").cyan().bold());
                true
            } else {
                eprintln!("{}", style("Error: Could not write to file").red());
                false
            }
        }
        Err(e) => {
            eprintln!("Error: Could not read file {}: {}", path.display(), e);
            false
        }
    }
}

/// Prints the ASCII branding logo.
fn show_branding() {
    println!("{}", style(ASCII).magenta().bold());
    println!(
        "\n{}",
        style("— A blazing-fast code health analyzer :)")
            .italic()
            .cyan()
    );
}
