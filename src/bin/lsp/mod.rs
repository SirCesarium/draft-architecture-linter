//! Backend implementation for the Sweet Language Server.

pub mod diag;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use swt::analyzer::analyze_content;
use swt::analyzer::ignore::get_disabled_rules;
use swt::languages::{Language, LanguageRegistry};
use swt::Config;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub workspace_root: Arc<RwLock<Option<PathBuf>>>,
}

impl Backend {
    pub async fn validate_document(&self, uri: Url, content: &str) {
        let Ok(path) = uri.to_file_path() else { return };
        if !Config::is_supported_file(&path) {
            return;
        }
        if let Some(ref root) = *self.workspace_root.read().await
            && !path.starts_with(root)
        {
            return;
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

        let diagnostics = diag::generate(&report, &config);
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        if let Some(root_uri) = params.root_uri
            && let Ok(root_path) = root_uri.to_file_path()
        {
            *self.workspace_root.write().await = Some(root_path);
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
