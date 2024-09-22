use super::{
    util::{create_code_block, Compiled, CompilerResult, LspRequest},
    LSPServer,
};
use crate::{
    compiler::program::ir::VariableKind,
    lsp::util::{find_block, get_processed_code, get_variable_at_position},
};
use lsp_types::{Hover, HoverParams};
use std::error::Error;

impl LSPServer {
    pub async fn handle_hover(
        &mut self,
        request: &LspRequest,
        params: HoverParams,
    ) -> Result<(), Box<dyn Error>> {
        let pos_params = params.text_document_position_params;
        let uri = pos_params.text_document.uri;
        let pos = pos_params.position;

        let text = self.file_contents.get(&uri).await?;

        let processed = get_processed_code(&text, uri.as_str());
        let CompilerResult::Compiled {
            lexed,
            compiled: Compiled { program, .. },
            ..
        } = processed
        else {
            self.write_empty_response(request.id).await?;
            return Ok(());
        };

        let Some((block, block_range)) = find_block(&lexed, &text, pos) else {
            self.write_empty_response(request.id).await?;
            return Ok(());
        };

        let variable = get_variable_at_position(&text, block, pos, &program);

        let Some(variable) = variable else {
            self.write_empty_response(request.id).await?;
            return Ok(());
        };

        // let kind = match &variable.typ {
        //     VariableKind::Normal(variable) => format!("Variable ({:?})", variable.typ),
        //     VariableKind::DeclaredFunction(function) => {
        //         let variables = function
        //             .parameter_variable_ids
        //             .iter()
        //             .map(|id| program.variables.get(*id).unwrap())
        //             .map(|variable| {
        //                 format!(
        //                     "{:?}: {:?}",
        //                     variable.identifier,
        //                     match &variable.typ {
        //                         VariableKind::Normal(variable) => &variable.typ,
        //                         VariableKind::DeclaredFunction(_) => unreachable!(),
        //                     }
        //                 )
        //             })
        //             .collect::<Vec<_>>()
        //             .join(", ");

        //         format!("Function ({variables})")
        //     }
        // };

        // let contents = lsp_types::HoverContents::Scalar(lsp_types::MarkedString::from_markdown(
        //     [
        //         create_code_block("rost", "let a = 5;").to_string(),
        //         format!("<{:?}> {:?} {kind}", variable.identifier, variable.scope).to_string(),
        //     ]
        //     .join("\n\n"),
        // ));

        // let result = Hover {
        //     contents,
        //     range: Some(block_range),
        // };

        // self.write_lsp_message(request.id, &result).await?;

        // Ok(())

        todo!();

    }
}
