//! Terminal-based report rendering.

use crate::FileReport;
use console::{Emoji, style};
use std::io::{self, BufWriter, Write};

/// Renders a summary of file reports to the terminal.
///
/// Supports a "quiet" mode for CI environments, providing a minimal output.
pub fn print_summary(reports: &[FileReport], quiet: bool) {
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    if quiet {
        print_quiet_summary(&mut handle, reports);
        let _ = handle.flush();
        return;
    }

    let candy = Emoji("🍬 ", "!");
    let lemon_emoji = Emoji("🍋 ", "X");

    let total = reports.len();
    let sweet_count = reports.iter().filter(|r| r.is_sweet).count();
    let bitter_count = total - sweet_count;

    let _ = writeln!(
        handle,
        "\n{}",
        style(" Results Summary ").bold().cyan().on_black()
    );
    let _ = writeln!(handle, "{}", style("─".repeat(60)).dim());

    for report in reports {
        render_file_row(&mut handle, report);
    }

    let _ = writeln!(handle, "{}", style("─".repeat(60)).dim());

    let summary_text =
        format!("Total: {total}  |  Sweet: {sweet_count}  |  Bitter: {bitter_count}");

    let _ = writeln!(
        handle,
        "\n{} {}\n",
        if bitter_count == 0 {
            candy
        } else {
            lemon_emoji
        },
        style(summary_text).bold()
    );

    render_final_message(&mut handle, bitter_count);
    let _ = handle.flush();
}

/// Renders a single row in the terminal report.
fn render_file_row<W: Write>(handle: &mut W, report: &FileReport) {
    let path_str = report.path.to_string_lossy();

    // Priority Emojis (Easter Eggs).
    let mut special_emoji = "";
    if report.imports == 67 {
        special_emoji = " 👐";
    } else if report.imports == 666 {
        special_emoji = " 🤘";
    } else if report.imports > 666 {
        special_emoji = " 🤮";
    }

    // Line Volume Emojis.
    let line_emoji = if special_emoji.is_empty() {
        if report.lines > 400 {
            " 🤕"
        } else if report.lines > 200 {
            " 🍋"
        } else {
            ""
        }
    } else {
        ""
    };

    let stats = format!(
        "{} lines · {} imports · depth {} · {:.1}% repeat{line_emoji}{special_emoji}",
        report.lines, report.imports, report.max_depth, report.repetition
    );

    if report.is_sweet {
        let _ = writeln!(
            handle,
            "{} {:<30} {}",
            style(" ✦ ").green().bold(),
            style(path_str).white(),
            style(stats).dim()
        );
    } else {
        let _ = writeln!(
            handle,
            "{} {:<30} {}",
            style(" ✘ ").red().bold(),
            style(path_str).magenta().bold(),
            style(stats).dim()
        );

        for (i, issue) in report.issues.iter().enumerate() {
            let connector = if i == report.issues.len() - 1 {
                " ╰─ "
            } else {
                " ├─ "
            };
            let _ = writeln!(
                handle,
                "    {}{}",
                style(connector).dim(),
                style(issue).yellow().italic()
            );
        }
        let _ = writeln!(handle);
    }
}

/// Renders a minimal summary for quiet mode.
fn print_quiet_summary<W: Write>(handle: &mut W, reports: &[FileReport]) {
    let bitter_count = reports.iter().filter(|r| !r.is_sweet).count();

    if bitter_count == 0 {
        return;
    }

    for report in reports {
        if !report.is_sweet {
            let _ = writeln!(
                handle,
                "{} {}: {}",
                style("BITTER").red().bold(),
                style(report.path.display()).white(),
                style(report.issues.join(", ")).yellow().italic()
            );
        }
    }

    let total = reports.len();
    let sweet_count = total - bitter_count;

    let _ = writeln!(
        handle,
        "\nSummary: {} files analyzed, {} sweet, {} bitter",
        style(total).bold(),
        style(sweet_count).green(),
        style(bitter_count).red().bold()
    );
}

/// Renders the final inspirational or warning message.
fn render_final_message<W: Write>(handle: &mut W, bitter_count: usize) {
    if bitter_count == 0 {
        let _ = writeln!(
            handle,
            "{}",
            style(" ✨ Your code is perfectly sweet! ✨ ")
                .green()
                .bold()
                .italic()
        );
    } else {
        let _ = writeln!(
            handle,
            "{}",
            style(" ⚠  Some files need a little more sugar...")
                .magenta()
                .bold()
        );
    }
}
