use super::LSPServer;
use crate::error::RostError;
use crate::lsp::util::{get_processed_code, pos_to_row_col};
use lsp_types::{Diagnostic, PublishDiagnosticsParams, Uri};
use std::error::Error;

impl LSPServer {
    fn get_diagnostics(&self, text: &str, errs: Vec<RostError>) -> Vec<Diagnostic> {
        eprintln!("Errors: {:?}", errs);

        errs.iter()
            .flat_map(|el| &el.elements)
            .map(|el| Diagnostic {
                range: pos_to_row_col(text, &el.pos),
                severity: None,
                code: None,
                code_description: None,
                source: None,
                message: el.message.clone(),
                related_information: None,
                tags: None,
                data: None,
            })
            .collect::<Vec<_>>()
    }

    pub async fn publish_diagnostics(
        &mut self,
        uri: &Uri,
        version: Option<i32>,
    ) -> Result<(), Box<dyn Error>> {
        let text = self.file_contents.get(uri).await?;

        let diagnostics = match get_processed_code(&text, uri.as_str()) {
            Ok(_) => vec![],
            Err(errs) => self.get_diagnostics(&text, errs),
        };

        let params = PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics,
            version,
        };

        self.write_lsp_notification("textDocument/publishDiagnostics", &params)
            .await?;

        Ok(())
    }
}
