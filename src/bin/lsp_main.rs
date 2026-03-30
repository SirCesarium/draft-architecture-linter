//! Entry point for the sweet-analyzer-lsp binary.

#![deny(
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::absolute_paths
)]

mod lsp;

use std::sync::Arc;
use tokio::io::{stdin, stdout};
use tokio::sync::RwLock;
use tower_lsp::{LspService, Server};

use crate::lsp::Backend;

#[tokio::main]
async fn main() {
    let (service, socket) = LspService::new(|client| Backend {
        client,
        workspace_root: Arc::new(RwLock::new(None)),
    });
    Server::new(stdin(), stdout(), socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::result::Result as StdResult;
    use tower_lsp::lsp_types::{InitializeParams, Url};
    use tower_lsp::{LanguageServer, LspService};

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
        service.inner().validate_document(uri, "test").await;
        Ok(())
    }
}
