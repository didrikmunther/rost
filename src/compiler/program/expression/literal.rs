use crate::{
    compiler::{
        error::CompilerError,
        program::{
            builder::Builder,
            ir::{Instruction, InstructionKind, PrimitiveValue, ValueKind},
            Program,
        },
    },
    lexer::Literal,
    parser::definition::Expression,
};

impl Program {
    fn handle_string_literal(
        &mut self,
        expression: &Expression,
        string: &str,
    ) -> Result<Builder, CompilerError> {
        let global_data_index = self.global_data.len();
        self.global_data
            .push(PrimitiveValue::String(string.to_string()));

        Ok(Builder::default().push(Instruction::new(
            expression.pos.clone(),
            InstructionKind::Push(ValueKind::GlobalData(global_data_index)),
        )))
    }

    fn handle_int_literal(
        &mut self,
        expression: &Expression,
        int: i32,
    ) -> Result<Builder, CompilerError> {
        Ok(Builder::default().push(Instruction::new(
            expression.pos.clone(),
            InstructionKind::Push(ValueKind::Primitive(PrimitiveValue::Int(int))),
        )))
    }

    fn handle_bool_literal(
        &mut self,
        _expression: &Expression,
        _b: bool,
    ) -> Result<Builder, CompilerError> {
        // Ok(Builder::default().push(Instruction::new(
        //     expression.pos.clone(),
        //     InstructionKind::Push(OperandValue::Int(if b { 1 } else { 0 })),
        // )))

        todo!()
    }

    pub fn handle_literal(
        &mut self,
        expression: &Expression,
        literal: &Literal,
    ) -> Result<Builder, CompilerError> {
        match literal {
            Literal::String(s) => self.handle_string_literal(expression, s),
            Literal::Int(i) => self.handle_int_literal(expression, *i),
            Literal::Bool(b) => self.handle_bool_literal(expression, *b),
        }
    }
}
