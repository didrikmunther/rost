use std::error::Error;

use lsp_types::{
    CodeActionProviderCapability, HoverProviderCapability, InitializeParams, InitializeResult,
    InitializedParams, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmptyClientNotification;

use super::{util::LspRequest, LSPServer};

impl LSPServer {
    pub async fn handle_initialize(
        &mut self,
        request: &LspRequest,
        _params: InitializeParams,
    ) -> Result<(), Box<dyn Error>> {
        let result = InitializeResult {
            capabilities: ServerCapabilities {
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(lsp_types::OneOf::Left(true)),
                ..Default::default()
            },
            server_info: None,
        };

        self.write_lsp_message(request.id, &result).await?;

        Ok(())
    }

    pub async fn handle_initialized(
        &mut self,
        _request: &LspRequest,
        _params: InitializedParams,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub async fn handle_exit(
        &mut self,
        _request: &LspRequest,
        _params: EmptyClientNotification,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
