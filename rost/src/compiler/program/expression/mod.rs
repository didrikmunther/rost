use crate::{
    compiler::error::{CompilerError, CompilerErrorKind, WrongType},
    lexer::{Keyword, Literal},
    parser::definition::{Binary, Expression, ExpressionKind, Primary},
};

use super::{
    builder::Builder,
    ir::{Arithmetic, Instruction, InstructionKind},
    typ::{ExpressedType, TypeKind},
    Program,
};

mod identifier;
mod literal;
mod primary;

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

    // fn handle_unary(
    //     &mut self,
    //     expression: &Expression,
    //     unary: &Unary,
    // ) -> Result<Builder, CompilerError> {
    //     match unary.operator {
    //         Keyword::Ampersand => self.handle_ref(&unary.expr),
    //         Keyword::Asterix => self.handle_deref(expression, &unary.expr),
    //         _ => todo!("Not supported"),
    //     }
    // }

    pub fn get_intrinstic_type(&self, identifier: &str) -> ExpressedType {
        ExpressedType {
            id: *self.get_scope().type_lookup.get(identifier).unwrap(),
            arguments: None,
        }
    }

    pub fn infer_type(&mut self, expr: &Expression) -> Result<ExpressedType, CompilerError> {
        match &expr.kind {
            ExpressionKind::Primary(primary) => match primary {
                Primary::Identifier(ref identifier) => {
                    let Some(variable_id) = self.get_variable(identifier) else {
                        return CompilerErrorKind::UndefinedVariable(identifier.clone())
                            .at_pos(&expr.pos)
                            .into();
                    };

                    let variable = self.variables.get(variable_id.get_id()).unwrap();

                    Ok(variable.typ.clone())
                }
                Primary::Literal(literal) => Ok(match literal {
                    Literal::Int(_) => self.get_intrinstic_type("int"),
                    Literal::String(_) => self.get_intrinstic_type("str"),
                    Literal::Bool(_) => self.get_intrinstic_type("bool"),
                }),
            },
            ExpressionKind::Binary(binary) => {
                let left = self.infer_type(&binary.left)?;
                let right = self.infer_type(&binary.right)?;

                let is_unknown =
                    [&left, &right]
                        .iter()
                        .any(|v| match &self.types.get(v.id).unwrap().kind {
                            TypeKind::Intrinsic { identifier } => identifier == "unknown",
                            _ => false,
                        });

                if !is_unknown && left != right {
                    self.add_error(CompilerError::new(
                        binary.left.pos.clone(),
                        CompilerErrorKind::WrongBinaryExpressionTypes {
                            got: WrongType::from_expressed_type(left.clone(), self),
                            expected: WrongType::from_expressed_type(right, self),
                            expected_pos: binary.right.pos.clone(),
                            operator: binary.operator,
                            operator_pos: binary.operator_pos.clone(),
                        },
                    ));

                    return Ok(self.get_intrinstic_type("unknown"));
                }

                Ok(left)

                // if left == right {
                //     Ok(left)
                // } else {
                //     Err(CompilerError::new(
                //         binary.left.pos.clone(),
                //         CompilerErrorKind::WrongBinaryExpressionTypes {
                //             got: WrongType::from_expressed_type(left, self),
                //             expected: WrongType::from_expressed_type(right, self),
                //             expected_pos: binary.right.pos.clone(),
                //             operator: binary.operator,
                //             operator_pos: binary.operator_pos.clone(),
                //         },
                //     ))
                // }

                // let inferred = self.infer_binary_result_type(&left, &right, binary.operator);
                // let Some(typ) = inferred else {
                //     return Err(CompilerError::new(
                //         binary.left.pos.clone(),
                //         CompilerErrorKind::WrongBinaryExpressionTypes {
                //             got: left,
                //             expected: right,
                //             expected_pos: binary.right.pos.clone(),
                //             operator: binary.operator,
                //             operator_pos: binary.operator_pos.clone(),
                //         },
                //     ));
                // };

                // Ok(typ)
            }
            _ => {
                eprintln!("Unknown expression: {:?}", expr);

                Ok(self.get_intrinstic_type("unknown"))
            } // ExpressionKind::ArrayIndex(index) => {
              //     let expr_type = self.infer_type(&index.left)?;

              //     match expr_type {
              //         VariableType::Pointer(pointer_type) => Ok(*pointer_type),
              //         _ => Err(CompilerError::new(
              //             index.left.pos.clone(),
              //             CompilerErrorKind::DereferenceNonPointer(expr_type),
              //         )),
              //     }
              // }
              // ExpressionKind::Unary(unary) => {
              //     let expr_type = self.infer_type(&unary.expr)?;

              //     match unary.operator {
              //         Keyword::Ampersand => Ok(VariableType::Pointer(Box::new(expr_type))),
              //         Keyword::Asterix => {
              //             let VariableType::Pointer(typ) = expr_type else {
              //                 return Err(CompilerError::new(
              //                     unary.operator_pos.clone(),
              //                     CompilerErrorKind::DereferenceNonPointer(expr_type),
              //                 ));
              //             };

              //             Ok(*typ)
              //         }
              //         _ => todo!("Not supported"),
              //     }
              // }
              // ExpressionKind::FunctionCall(call) => {
              //     let identifier = call.left.get_string().unwrap().to_string();

              //     let Some(function) = self.get_variable(&identifier) else {
              //         return Err(CompilerError::new(
              //             call.left.pos.clone(),
              //             CompilerErrorKind::UndefinedFunction(identifier),
              //         ));
              //     };

              //     let VariableType::Function(function_id) = function.typ else {
              //         todo!("Variable is not a function");
              //     };

              //     let Some(return_type) = &self.functions.get(function_id).unwrap().return_type
              //     else {
              //         todo!("No return type for function");
              //     };

              //     Ok(return_type.clone())
              // }
              // ExpressionKind::StructConstruction(sconst) => {
              //     Ok(self.get_variable(&sconst.identifier).unwrap().typ.clone())
              // }
              // ExpressionKind::MemberAccess(access) => Ok(self
              //     .get_struct_field_type(&access.left, &access.member)?
              //     .typ
              //     .clone()),
        }
    }

    fn handle_binary(
        &mut self,
        expression: &Expression,
        binary: &Binary,
    ) -> Result<Builder, CompilerError> {
        let operation = Self::get_arithmetic_operation(binary.operator);
        let _ = self.infer_type(expression)?;
        let _right = self.infer_type(&binary.right)?;
        let _left = self.infer_type(&binary.left)?;

        let instruction_kind = match operation {
            Arithmetic::Add => InstructionKind::IntAdd,
            Arithmetic::Multiply => InstructionKind::IntMul,
            _ => todo!(),
        };

        Ok(Builder::new()
            .append(self.handle_expression(&binary.right)?)
            .append(self.handle_expression(&binary.left)?)
            .push(Instruction::new(expression.pos.clone(), instruction_kind)))

        // let _ = self.infer_type(expression)?;
        // let right = self.infer_type(&binary.right)?;
        // let left = self.infer_type(&binary.left)?;

        // match (left, right, &operation) {
        //     // (
        //     //     VariableType::Pointer(p_left),
        //     //     VariableType::Pointer(p_right),
        //     //     Arithmetic::Add
        //     //     | Arithmetic::Subtract
        //     //     | Arithmetic::Equality
        //     //     | Arithmetic::GreaterThan
        //     //     | Arithmetic::LessThan,
        //     // ) => {
        //     //     if p_left != p_right {
        //     //         todo!("Not supported")
        //     //     }

        //     //     Ok(Builder::new()
        //     //         .append(self.handle_expression(&binary.right)?)
        //     //         .append(self.handle_expression(&binary.left)?)
        //     //         .push(Procedure::new(
        //     //             expression.pos.clone(),
        //     //             ProcedureKind::Arithmetic(operation, RegisterSize::B64),
        //     //         )))
        //     // }
        //     (
        //         VariableType::Value(Keyword::Int | Keyword::Char),
        //         VariableType::Value(Keyword::Int | Keyword::Char),
        //         _,
        //     ) => {
        //         // let register_size_left = RegisterSize::get_register(Self::get_type_size(
        //         //     &self.infer_type(&binary.left)?,
        //         // ));
        //         // let register_size_right = RegisterSize::get_register(Self::get_type_size(
        //         //     &self.infer_type(&binary.right)?,
        //         // ));
        //         // let register_size = register_size_left.get_smallest(register_size_right);

        //         Ok(Builder::new()
        //             .append(self.handle_expression(&binary.right)?)
        //             .append(self.handle_expression(&binary.left)?)
        //             .push(Procedure::new(
        //                 expression.pos.clone(),
        //                 ProcedureKind::Arithmetic(operation, register_size),
        //             )))
        //     }
        //     // (
        //     //     VariableType::Pointer(pointer_type),
        //     //     VariableType::Value(Keyword::Int),
        //     //     Arithmetic::Add | Arithmetic::Subtract,
        //     // ) => Ok(Builder::new()
        //     //     .append(self.handle_expression(&binary.right)?)
        //     //     .push(Procedure::new(
        //     //         expression.pos.clone(),
        //     //         ProcedureKind::Push(OperandValue::Int(
        //     //             Self::get_type_size(&pointer_type) as i32
        //     //         )),
        //     //     ))
        //     //     .push(Procedure::new(
        //     //         expression.pos.clone(),
        //     //         ProcedureKind::Arithmetic(Arithmetic::Multiply, RegisterSize::B64),
        //     //     ))
        //     //     .append(self.handle_expression(&binary.left)?)
        //     //     .push(Procedure::new(
        //     //         expression.pos.clone(),
        //     //         ProcedureKind::Arithmetic(operation, RegisterSize::B64),
        //     //     ))),
        //     _ => todo!("Not supported"),
        // }
    }

    pub fn handle_expression(&mut self, expression: &Expression) -> Result<Builder, CompilerError> {
        match &expression.kind {
            ExpressionKind::FunctionCall(fcall) => self.handle_function_call(expression, fcall),
            // ExpressionKind::StructConstruction(sconst) => {
            //     self.handle_struct_construction(expression, sconst)
            // }
            // ExpressionKind::ArrayIndex(index) => self.handle_array_index(expression, index),
            // ExpressionKind::MemberAccess(access) => self.handle_member_access(expression, access),
            ExpressionKind::Primary(primary) => self.handle_primary(expression, primary),
            // ExpressionKind::Unary(unary) => self.handle_unary(expression, unary),
            ExpressionKind::Binary(binary) => self.handle_binary(expression, binary),
            _ => {
                eprintln!("Expression: {:?}", expression);
                todo!()
            }
        }
    }
}
