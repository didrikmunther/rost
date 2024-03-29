use crate::{
    compiler::{
        builder::Builder,
        definition::{Arithmetic, OperandValue, Procedure, ProcedureKind},
        error::CompilerError,
        program::Program,
        scope::variable::VariableType,
    },
    lexer::Keyword,
    parser::definition::{Binary, Expression, ExpressionKind, Unary},
};

use super::definition::RegisterSize;

mod identifier;
mod literal;
mod primary;
mod reference;

impl Program {
    fn get_arithmetic_operation(operator: Keyword) -> Arithmetic {
        match operator {
            Keyword::Plus => Arithmetic::Add,
            Keyword::Minus => Arithmetic::Subtract,
            Keyword::Asterix => Arithmetic::Multiply,
            Keyword::Slash => Arithmetic::Divide,
            Keyword::LessThan => Arithmetic::LessThan,
            Keyword::GreaterThan => Arithmetic::GreaterThan,
            Keyword::Equality => Arithmetic::Equality,
            _ => todo!("Not supported"),
        }
    }

    fn handle_unary(
        &mut self,
        expression: &Expression,
        unary: &Unary,
    ) -> Result<Builder, CompilerError> {
        match unary.operator {
            Keyword::Ampersand => self.handle_ref(&unary.expr),
            Keyword::Asterix => self.handle_deref(expression, &unary.expr),
            _ => todo!("Not supported"),
        }
    }

    fn handle_binary(
        &mut self,
        expression: &Expression,
        binary: &Binary,
    ) -> Result<Builder, CompilerError> {
        let operation = Self::get_arithmetic_operation(binary.operator);
        let _ = self.infer_type(expression)?;
        let right = self.infer_type(&binary.right)?;
        let left = self.infer_type(&binary.left)?;

        match (left, right, &operation) {
            (
                VariableType::Pointer(p_left),
                VariableType::Pointer(p_right),
                Arithmetic::Add
                | Arithmetic::Subtract
                | Arithmetic::Equality
                | Arithmetic::GreaterThan
                | Arithmetic::LessThan,
            ) => {
                if p_left != p_right {
                    todo!("Not supported")
                }

                Ok(Builder::new()
                    .append(self.handle_expression(&binary.right)?)
                    .append(self.handle_expression(&binary.left)?)
                    .push(Procedure::new(
                        expression.pos.clone(),
                        ProcedureKind::Arithmetic(operation, RegisterSize::B64),
                    )))
            }
            (
                VariableType::Value(Keyword::Int | Keyword::Char),
                VariableType::Value(Keyword::Int | Keyword::Char),
                _,
            ) => {
                let register_size_left = RegisterSize::get_register(Self::get_type_size(
                    &self.infer_type(&binary.left)?,
                ));
                let register_size_right = RegisterSize::get_register(Self::get_type_size(
                    &self.infer_type(&binary.right)?,
                ));
                let register_size = register_size_left.get_smallest(register_size_right);

                Ok(Builder::new()
                    .append(self.handle_expression(&binary.right)?)
                    .append(self.handle_expression(&binary.left)?)
                    .push(Procedure::new(
                        expression.pos.clone(),
                        ProcedureKind::Arithmetic(operation, register_size),
                    )))
            }
            (
                VariableType::Pointer(pointer_type),
                VariableType::Value(Keyword::Int),
                Arithmetic::Add | Arithmetic::Subtract,
            ) => Ok(Builder::new()
                .append(self.handle_expression(&binary.right)?)
                .push(Procedure::new(
                    expression.pos.clone(),
                    ProcedureKind::Push(OperandValue::Int(
                        Self::get_type_size(&pointer_type) as i32
                    )),
                ))
                .push(Procedure::new(
                    expression.pos.clone(),
                    ProcedureKind::Arithmetic(Arithmetic::Multiply, RegisterSize::B64),
                ))
                .append(self.handle_expression(&binary.left)?)
                .push(Procedure::new(
                    expression.pos.clone(),
                    ProcedureKind::Arithmetic(operation, RegisterSize::B64),
                ))),
            _ => todo!("Not supported"),
        }
    }

    pub fn handle_expression(&mut self, expression: &Expression) -> Result<Builder, CompilerError> {
        match &expression.kind {
            ExpressionKind::FunctionCall(fcall) => self.handle_function_call(expression, fcall),
            ExpressionKind::StructConstruction(sconst) => {
                self.handle_struct_construction(expression, sconst)
            }
            ExpressionKind::ArrayIndex(index) => self.handle_array_index(expression, index),
            ExpressionKind::MemberAccess(access) => self.handle_member_access(expression, access),
            ExpressionKind::Primary(primary) => self.handle_primary(expression, primary),
            ExpressionKind::Unary(unary) => self.handle_unary(expression, unary),
            ExpressionKind::Binary(binary) => self.handle_binary(expression, binary),
        }
    }
}
