use crate::parser::definition::FunctionDeclaration;

use super::{builder::Builder, error::CompilerError, program::Program};

impl Program {
    pub fn handle_function_declaration(
        &mut self,
        fdec: &FunctionDeclaration,
    ) -> Result<Builder, CompilerError> {
        let mut builder = Builder::new();

        todo!()

        // for arg in &fcall.args {
        //     let expr = self.handle_expression(arg)?;
        //     builder = builder.append(expr);
        // }

        // builder = builder.push(Procedure::new(
        //     expression.pos.clone(),
        //     ProcedureKind::SystemCall(SystemCall {
        //         nargs: fcall.args.len(),
        //         identifier: fcall.identifier.clone(),
        //     }),
        // ));

        // Ok(builder)
    }
}
