use std::error::Error;

use lsp_types::DidChangeTextDocumentParams;

use super::{util::LspRequest, LSPServer};

impl LSPServer {
    pub async fn handle_did_change(
        &mut self,
        _request: &LspRequest,
        _params: DidChangeTextDocumentParams,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
