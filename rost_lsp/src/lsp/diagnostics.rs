use super::util::{Compiled, CompilerResult};
use super::LSPServer;
use crate::lsp::util::{get_processed_code, pos_to_row_col};
use lsp_types::{Diagnostic, PublishDiagnosticsParams, Uri};
use rost::error::{RostError, RostErrorElement};
use std::error::Error;

fn create_error_message(err: &RostError, element: &RostErrorElement) -> Vec<String> {
    vec![format!("{}", err.kind), format!("{}", element.message)]
}

impl LSPServer {
    fn get_diagnostics(&self, text: &str, errs: Vec<RostError>) -> Vec<Diagnostic> {
        eprintln!("Errors: {:?}", errs);

        let mut diagnostics = vec![];

        for err in errs {
            for el in &err.elements {
                diagnostics.push(Diagnostic {
                    range: pos_to_row_col(text, &el.pos),
                    severity: None,
                    code: None,
                    code_description: None,
                    source: None,
                    message: create_error_message(&err, el).join("\n\n"),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }

        diagnostics
    }

    pub async fn publish_diagnostics(
        &mut self,
        uri: &Uri,
        version: Option<i32>,
    ) -> Result<(), Box<dyn Error>> {
        let text = self.file_contents.get(uri).await?;

        let diagnostics = match get_processed_code(&text, uri.as_str()) {
            CompilerResult::Lexed(Err(err)) => self.get_diagnostics(&text, vec![err]),
            CompilerResult::Parsed {
                parsed: Err(err), ..
            } => self.get_diagnostics(&text, vec![err]),
            CompilerResult::Compiled {
                compiled: Compiled { errors, .. },
                ..
            } => self.get_diagnostics(&text, errors),
            _ => vec![],
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
