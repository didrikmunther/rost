use crate::parser::definition::{Declaration, FunctionDeclaration};

use super::{
    builder::Builder,
    definition::{FunctionDefinition, Procedure, ProcedureKind},
    error::CompilerError,
    program::Program,
};

impl Program {
    pub fn handle_function_declaration(
        &mut self,
        statement: &Declaration,
        fdec: &FunctionDeclaration,
    ) -> Result<Builder, CompilerError> {
        todo!()
        // let mut builder = Builder::new();

        // // for par in &fdec.parameters {
        // //     let expr = self.handle_expression(par)?;
        // //     builder = builder.append(expr);
        // // }

        // fdec.identifier

        // builder = builder.push(Procedure::new(
        //     statement.pos.clone(),
        //     ProcedureKind::FunctionDefinition(FunctionDefinition {
        //         npars: fdec.parameters,
        //         identifier: fcall.identifier.clone(),
        //     }),
        // ));

        // Ok(builder)
    }
}
