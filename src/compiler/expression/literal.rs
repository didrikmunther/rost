use crate::{
    compiler::{
        builder::Builder,
        definition::{GlobalData, OperandValue, Procedure, ProcedureKind},
        error::{CompilerError, CompilerErrorKind},
        program::Program,
    },
    lexer::Literal,
    parser::definition::Expression,
};

impl Program {
    fn handle_string_literal(
        &mut self,
        expression: &Expression,
        string: &String,
    ) -> Result<Builder, CompilerError> {
        self.global_data.push(GlobalData {
            content: string.clone(),
        });
        let last_index = self.global_data.len() - 1;

        Ok(Builder::new().push(Procedure::new(
            expression.pos.clone(),
            ProcedureKind::Push(OperandValue::ByteLocation(last_index)),
        )))
    }

    fn handle_int_literal(
        &mut self,
        expression: &Expression,
        int: i32,
    ) -> Result<Builder, CompilerError> {
        Ok(Builder::new().push(Procedure::new(
            expression.pos.clone(),
            ProcedureKind::Push(OperandValue::Int(int)),
        )))
    }

    pub fn handle_literal(
        &mut self,
        expression: &Expression,
        literal: &Literal,
    ) -> Result<Builder, CompilerError> {
        match literal {
            Literal::String(s) => self.handle_string_literal(expression, s),
            Literal::Int(i) => self.handle_int_literal(expression, *i),
            _ => {
                return Err(CompilerError::new(
                    expression.pos.clone(),
                    CompilerErrorKind::Unimplemented(format!("{:?}", literal)),
                ));
            }
        }
    }
}
