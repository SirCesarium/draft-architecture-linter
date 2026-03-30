#![deny(
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::absolute_paths
)]

use serde_json::to_value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use swt::analyzer::analyze_content;
use swt::analyzer::ignore::get_disabled_rules;
use swt::languages::{Language, LanguageRegistry};
use swt::{Config, FileReport, Severity};
use tokio::io::{stdin, stdout};
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, CodeActionProviderCapability,
    CodeActionResponse, Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity,
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, InitializeResult,
    InitializedParams, Location, MessageType, Position, Range, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit, Url, WorkspaceEdit,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    workspace_root: Arc<RwLock<Option<PathBuf>>>,
}

impl Backend {
    async fn validate_document(&self, uri: Url, content: &str) {
        let Ok(path) = uri.to_file_path() else {
            return;
        };

        if !Config::is_supported_file(&path) {
            return;
        }

        // Only analyze files within the workspace root to avoid noisy warnings on external files.
        if let Some(ref root) = *self.workspace_root.read().await {
            if !path.starts_with(root) {
                return;
            }
        }

        let config = Config::load(&path);
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        let thresholds = config.get_thresholds(extension);

        let disabled_rules = get_disabled_rules(content);
        let report = analyze_content(
            content,
            extension,
            &thresholds,
            &path,
            &config,
            &disabled_rules,
            true,
        );

        let mut diagnostics = Vec::new();

        for issue in &report.issues {
            let rule = if issue.message.contains("File too long") {
                "max-lines"
            } else if issue.message.contains("Too many imports") {
                "max-imports"
            } else if issue.message.contains("Excessive nesting") {
                "max-depth"
            } else if issue.message.contains("repetition") {
                "max-repetition"
            } else {
                "unknown"
            };

            let severity = match config.thresholds.severities.get(rule) {
                Severity::Error => DiagnosticSeverity::ERROR,
                Severity::Warning => DiagnosticSeverity::WARNING,
            };

            diagnostics.push(Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 80)),
                severity: Some(severity),
                message: format!("🍬 Sweet: {}", issue.message),
                source: Some("sweet".to_string()),
                data: Some(to_value(rule).unwrap_or_default()),
                ..Default::default()
            });
        }

        for (line, depth) in &report.deep_lines {
            #[allow(clippy::cast_possible_truncation)]
            let l = (*line as u32).saturating_sub(1);
            let severity = match config.thresholds.severities.get("max-depth") {
                Severity::Error => DiagnosticSeverity::ERROR,
                Severity::Warning => DiagnosticSeverity::WARNING,
            };

            diagnostics.push(Diagnostic {
                range: Range::new(Position::new(l, 0), Position::new(l, 80)),
                severity: Some(severity),
                message: format!("🍬 Sweet: Excessive nesting depth: {depth}"),
                source: Some("sweet".to_string()),
                data: Some(to_value("max-depth").unwrap_or_default()),
                ..Default::default()
            });
        }

        Self::report_duplicates(&report, &config, &mut diagnostics);

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    fn report_duplicates(report: &FileReport, config: &Config, diagnostics: &mut Vec<Diagnostic>) {
        for duplicate in &report.duplicates {
            #[allow(clippy::cast_possible_truncation)]
            let start_line = (duplicate.line as u32).saturating_sub(1);
            #[allow(clippy::cast_possible_truncation)]
            let line_count = duplicate.content.lines().count() as u32;
            let end_line = start_line + line_count.saturating_sub(1);

            let mut related_information = Vec::new();
            for (other_path, other_line) in &duplicate.occurrences {
                if let Ok(other_uri) = Url::from_file_path(other_path) {
                    #[allow(clippy::cast_possible_truncation)]
                    let l = (*other_line as u32).saturating_sub(1);
                    related_information.push(DiagnosticRelatedInformation {
                        location: Location::new(
                            other_uri,
                            Range::new(Position::new(l, 0), Position::new(l, 80)),
                        ),
                        message: format!("Duplicate found here (line {other_line})"),
                    });
                }
            }

            let severity = match config.thresholds.severities.get("max-repetition") {
                Severity::Error => DiagnosticSeverity::ERROR,
                Severity::Warning => DiagnosticSeverity::WARNING,
            };

            diagnostics.push(Diagnostic {
                range: Range::new(Position::new(start_line, 0), Position::new(end_line, 80)),
                severity: Some(severity),
                message: format!(
                    "🍬 Sweet: Code duplication detected! (repeated in {} other places)",
                    duplicate.occurrences.len()
                ),
                source: Some("sweet".to_string()),
                related_information: Some(related_information),
                data: Some(to_value("max-repetition").unwrap_or_default()),
                ..Default::default()
            });
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        if let Some(root_uri) = params.root_uri {
            if let Ok(root_path) = root_uri.to_file_path() {
                *self.workspace_root.write().await = Some(root_path);
            }
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Sweet LSP server initialized!")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.validate_document(params.text_document.uri, &params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.first() {
            self.validate_document(params.text_document.uri, &change.text)
                .await;
        }
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let mut actions = Vec::new();

        let Ok(path) = params.text_document.uri.to_file_path() else {
            return Ok(None);
        };

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        let registry = LanguageRegistry::get();
        let comment = registry
            .get_by_extension(extension)
            .and_then(Language::line_comment)
            .unwrap_or("//");

        for diagnostic in params.context.diagnostics {
            if let Some(rule) = diagnostic.data.as_ref().and_then(|v| v.as_str()) {
                if rule == "unknown" {
                    continue;
                }

                let title = format!("🍬 Disable rule '{rule}' for this file");
                let edit = TextEdit::new(
                    Range::new(Position::new(0, 0), Position::new(0, 0)),
                    format!("{comment} @swt-disable {rule}\n"),
                );

                let mut changes = HashMap::new();
                changes.insert(params.text_document.uri.clone(), vec![edit]);

                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title,
                    kind: Some(CodeActionKind::QUICKFIX),
                    edit: Some(WorkspaceEdit {
                        changes: Some(changes),
                        ..Default::default()
                    }),
                    diagnostics: Some(vec![diagnostic]),
                    ..Default::default()
                }));
            }
        }

        Ok(Some(actions))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        workspace_root: Arc::new(RwLock::new(None)),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::result::Result as StdResult;
    use tower_lsp::LspService;

    #[tokio::test]
    async fn test_initialization() -> StdResult<(), Box<dyn Error>> {
        let (service, _) = LspService::new(|client| Backend {
            client,
            workspace_root: Arc::new(RwLock::new(None)),
        });
        let params = InitializeParams::default();
        let result = service.inner().initialize(params).await?;
        assert!(result.capabilities.text_document_sync.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_unsupported_file() -> StdResult<(), Box<dyn Error>> {
        let (service, _) = LspService::new(|client| Backend {
            client,
            workspace_root: Arc::new(RwLock::new(None)),
        });
        let uri = Url::parse("file:///test.txt")?;
        // Should not panic or return error, just skip
        service.inner().validate_document(uri, "test").await;
        Ok(())
    }
}
