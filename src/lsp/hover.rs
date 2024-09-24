use super::{
    util::{create_code_block, Compiled, CompilerResult, LspRequest},
    LSPServer,
};
use crate::{
    compiler::program::ir::{FunctionTypeKind, TypeKind, VariableKind},
    lsp::util::{find_block, get_processed_code, get_variable_at_position},
};
use lsp_types::{Hover, HoverParams};
use std::{error::Error, fmt::format};

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

        let typ = program.types.get(variable.typ.id).unwrap();

        eprintln!("kind: {:?}", typ.kind);

        let type_visualization = match &typ.kind {
            TypeKind::Generic { identifier } => vec![identifier.into()],
            TypeKind::Intrinsic { .. } => vec![
                create_code_block(
                    "rost",
                    &format!("let {}: {};", variable.identifier, &typ.format(&program)),
                )
                .to_string(),
                // variable.identifier.to_string(),
                // program
                //     .reverse_type_lookup
                //     .get(&variable.typ.id)
                //     .map(|s| s.as_str())
                //     .unwrap_or("")
                //     .into(),
            ],
            TypeKind::Function(FunctionTypeKind {
                parameter_type_ids,
                vararg_parameter_type_id,
                declaration_pos,
            }) => vec![
                // create_code_block("rost", &format!("{}()", variable.identifier)).to_string()
                create_code_block(
                    "rost",
                    &format!("{}{}", variable.identifier, &typ.format(&program)),
                )
                .to_string(),
            ],
            TypeKind::UserDefined {
                identifier,
                declaration_pos,
            } => vec!["user defined".into()],
        };

        // let kind = match typ.kind {
        //     _ => format!("{:?}", typ.kind),
        //     // VariableKind::Normal(variable) => format!("Variable ({:?})", variable.typ),
        //     // VariableKind::DeclaredFunction(function) => {
        //     //     let variables = function
        //     //         .parameter_variable_ids
        //     //         .iter()
        //     //         .map(|id| program.variables.get(*id).unwrap())
        //     //         .map(|variable| {
        //     //             format!(
        //     //                 "{:?}: {:?}",
        //     //                 variable.identifier,
        //     //                 match &variable.typ {
        //     //                     VariableKind::Normal(variable) => &variable.typ,
        //     //                     VariableKind::DeclaredFunction(_) => unreachable!(),
        //     //                 }
        //     //             )
        //     //         })
        //     //         .collect::<Vec<_>>()
        //     //         .join(", ");

        //     //     format!("Function ({variables})")
        //     // }
        // };

        let contents = lsp_types::HoverContents::Scalar(lsp_types::MarkedString::from_markdown(
            type_visualization.join("\n\n"),
        ));

        let result = Hover {
            contents,
            range: Some(block_range),
        };

        self.write_lsp_message(request.id, &result).await?;

        Ok(())
    }
}
