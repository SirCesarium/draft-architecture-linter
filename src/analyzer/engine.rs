//! High-level analysis orchestration and parallel file processing.

use crate::{Config, FileReport};
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::PathBuf;
use std::time::Duration;

/// The `AnalysisEngine` orchestrates the collection and parallel analysis of project files.
pub struct AnalysisEngine {
    root: PathBuf,
    config: Config,
}

impl AnalysisEngine {
    /// Creates a new `AnalysisEngine` instance.
    #[must_use]
    pub const fn new(root: PathBuf, config: Config) -> Self {
        Self { root, config }
    }

    /// Recursively collects all supported files within the root directory.
    ///
    /// Utilizes a parallel walker for high-speed file system discovery.
    #[must_use]
    pub fn collect_files(&self, quiet: bool) -> Vec<PathBuf> {
        let spinner = if quiet {
            None
        } else {
            let sp = ProgressBar::new_spinner();
            sp.set_style(
                ProgressStyle::with_template("{spinner:.magenta} {msg}")
                    .expect("Valid template")
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            );
            sp.set_message("Discovering project files...");
            sp.enable_steady_tick(Duration::from_millis(80));
            Some(sp)
        };

        let mut walk_builder = WalkBuilder::new(&self.root);
        for exclude in &self.config.exclude {
            walk_builder.add_ignore(exclude);
        }

        let (tx, rx) = std::sync::mpsc::channel();

        walk_builder.build_parallel().run(|| {
            let tx = tx.clone();
            Box::new(move |result| {
                if let Some(entry) = result.ok().filter(|e| Config::is_supported_file(e.path())) {
                    let _ = tx.send(entry.path().to_path_buf());
                }
                ignore::WalkState::Continue
            })
        });

        drop(tx);

        let entries: Vec<PathBuf> = rx.into_iter().collect();

        if let Some(sp) = spinner {
            sp.finish_and_clear();
        }

        entries
    }

    /// Executes the analysis phase in parallel using the Rayon thread pool.
    #[must_use]
    pub fn run(&self, quiet: bool, show_progress: bool) -> Vec<FileReport> {
        let entries = self.collect_files(quiet);

        if entries.is_empty() {
            return Vec::new();
        }

        let pb = Self::create_progress_bar(entries.len(), quiet, show_progress);

        let mut reports: Vec<FileReport> = entries
            .par_iter()
            .filter_map(|path| {
                let res = super::analyze_file(path, &self.config);
                if let Some(ref pb) = pb {
                    pb.inc(1);
                    if let Some(ref r) = res {
                        pb.set_message(format!("{}", r.path.display()));
                    }
                }
                res
            })
            .collect();

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }

        Self::sort_reports(&mut reports);
        reports
    }

    /// Initializes a progress bar for the analysis phase.
    fn create_progress_bar(len: usize, quiet: bool, show_progress: bool) -> Option<ProgressBar> {
        if quiet || !show_progress {
            return None;
        }

        let pb = ProgressBar::new(len as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{prefix:>12.cyan.bold} [{bar:40.magenta/dim}] {pos}/{len} {msg}",
            )
            .expect("Valid template")
            .progress_chars("⭓⭔-"),
        );
        pb.set_prefix("Analyzing");
        Some(pb)
    }

    /// Sorts reports prioritized by "bitterness", issue count, and file volume.
    fn sort_reports(reports: &mut [FileReport]) {
        reports.sort_by(|a, b| {
            b.is_sweet
                .cmp(&a.is_sweet)
                .then_with(|| b.issues.len().cmp(&a.issues.len()))
                .then_with(|| b.lines.cmp(&a.lines))
        });
    }
}
