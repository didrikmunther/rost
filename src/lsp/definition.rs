use crate::lsp::util::{find_block, get_processed_code, get_variable_at_position, pos_to_row_col};
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};
use std::error::Error;

use super::{util::LspRequest, LSPServer};

impl LSPServer {
    pub async fn handle_goto_definition(
        &mut self,
        request: &LspRequest,
        params: GotoDefinitionParams,
    ) -> Result<(), Box<dyn Error>> {
        let pos_params = params.text_document_position_params;
        let pos = pos_params.position;
        let uri = pos_params.text_document.uri;
        let text = self.file_contents.get(&uri).await?;

        let Ok((lexed, _parsed, program)) = get_processed_code(&text, uri.as_str()) else {
            self.write_empty_response(request.id).await?;
            return Ok(());
        };

        let Some((block, _block_range)) = find_block(&lexed, &text, pos) else {
            self.write_empty_response(request.id).await?;
            return Ok(());
        };

        let variable = get_variable_at_position(&text, block, pos, &program);

        let Some(variable) = variable else {
            self.write_empty_response(request.id).await?;
            return Ok(());
        };

        let variable_pos = pos_to_row_col(&text, &variable.declaration_pos);

        let result = GotoDefinitionResponse::Scalar(Location {
            uri,
            range: variable_pos,
        });

        self.write_lsp_message(request.id, &result).await?;

        Ok(())
    }
}
