use lsp_types::CodeActionParams;
use rost::util::{get_processed_code, Compiled, CompilerResult};
use std::error::Error;

use super::{util::LspRequest, LSPServer};

impl LSPServer {
    pub async fn handle_code_action(
        &mut self,
        request: &LspRequest,
        params: CodeActionParams,
    ) -> Result<(), Box<dyn Error>> {
        let uri = params.text_document.uri;
        let text = self.file_contents.get(&uri).await?;

        let processed = get_processed_code(&text, uri.as_str());
        let CompilerResult::Compiled {
            lexed: _,
            compiled: Compiled { program: _, .. },
            ..
        } = processed
        else {
            self.write_empty_response(request.id).await?;
            return Ok(());
        };

        // eprintln!()

        // let Some((block, _block_range)) = find_block(&lexed, &text, pos) else {
        //     self.write_empty_response(request.id).await?;
        //     return Ok(());
        // };

        // let variable = get_variable_at_position(&text, block, pos, &program);

        // let Some(variable) = variable else {
        //     self.write_empty_response(request.id).await?;
        //     return Ok(());
        // };

        // let variable_pos = pos_to_row_col(&text, &variable.declaration_pos);

        // let result = GotoDefinitionResponse::Scalar(Location {
        //     uri,
        //     range: variable_pos,
        // });

        // self.write_lsp_message(request.id, &result).await?;

        Ok(())
    }
}
