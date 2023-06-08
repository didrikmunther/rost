use crate::{
    lexer::{Keyword, Literal},
    parser::{
        definition::{Declaration, Expression, ExpressionKind, Primary},
        types::{Type, TypeIdentifier},
    },
};

use super::{
    builder::Builder,
    error::{CompilerError, CompilerErrorKind},
    program::Program,
    scope::{
        variable::{StoredVariable, Variable, VariableLocation, VariableType},
        ProgramScope,
    },
};

impl Program {
    pub fn get_procedures(&mut self, content: &[Declaration]) -> Result<Builder, CompilerError> {
        content
            .iter()
            .fold(Ok(Builder::new()), |builder, declaration| {
                Ok(builder?.append(self.handle_declaration(declaration)?))
            })
    }

    pub fn get_variable(&self, identifier: &String) -> Option<&StoredVariable> {
        match &self.scope {
            ProgramScope::RootScope(scope) => scope.get_variable(identifier),
            ProgramScope::FunctionScope(scope) => scope.get_variable(identifier),
        }
    }

    /// Creates a variable in the current scope.
    /// Returns location of the created variable.
    pub fn create_variable(&mut self, identifier: String, variable: Variable) -> VariableLocation {
        match &mut self.scope {
            ProgramScope::RootScope(scope) => scope.create_variable(identifier, variable),
            ProgramScope::FunctionScope(scope) => scope.create_variable(identifier, variable),
        }
    }

    fn get_primitive_type_size(primitive: &Keyword) -> usize {
        match primitive {
            Keyword::Int | Keyword::Char => 8,
            _ => todo!("Not supported"),
        }
    }

    /// Return sizes in bytes
    // todo: maybe this is different depending on platform?
    pub fn get_type_size(typ: &VariableType) -> usize {
        match typ {
            VariableType::Pointer(_) => 8,
            VariableType::Value(typ) => Self::get_primitive_type_size(typ),
            VariableType::Function(_) => todo!("Not supported"),
            VariableType::Struct(s) => s.size,
        }
    }

    pub fn infer_binary_result_type(
        &self,
        left: &VariableType,
        right: &VariableType,
        operator: Keyword,
    ) -> Option<VariableType> {
        match (left, right) {
            (VariableType::Pointer(_), VariableType::Value(Keyword::Int)) => Some(left.clone()),
            (VariableType::Value(Keyword::Int), VariableType::Pointer(_)) => match operator {
                Keyword::Plus => Some(right.clone()),
                _ => None,
            },
            (VariableType::Value(left), VariableType::Value(right)) => {
                if left != right {
                    return None;
                }

                match operator {
                    Keyword::Plus | Keyword::Minus | Keyword::Slash | Keyword::Asterix => {
                        Some(VariableType::Value(*left))
                    }
                    Keyword::LessThan | Keyword::GreaterThan | Keyword::Equality => {
                        Some(VariableType::Value(Keyword::Bool))
                    }
                    _ => None,
                }
            }
            (VariableType::Pointer(left), VariableType::Pointer(right)) => {
                if left != right {
                    return None;
                }

                match operator {
                    Keyword::Plus | Keyword::Minus => {
                        if left == right {
                            Some(VariableType::Value(Keyword::Int))
                        } else {
                            None
                        }
                    }
                    Keyword::LessThan | Keyword::GreaterThan | Keyword::Equality => {
                        if left == right {
                            Some(VariableType::Value(Keyword::Bool))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn get_variable_type(&self, typ: &Type) -> VariableType {
        match typ.identifier {
            TypeIdentifier::Primitive(Keyword::Pointer) => {
                let Some(children) = &typ.children else {
                    todo!("Pointer type requires 1 child type");
                };

                if children.len() != 1 {
                    todo!("Wrong amount of type arguments for pointer type");
                }

                let inner = children.get(0).unwrap();
                VariableType::Pointer(Box::new(self.get_variable_type(inner)))
            }
            TypeIdentifier::Primitive(primitive) => VariableType::Value(primitive),
            TypeIdentifier::Struct(ref s) => self.get_variable(s).unwrap().typ.clone(),
        }
    }

    pub fn infer_type(&self, expr: &Expression) -> Result<VariableType, CompilerError> {
        match &expr.kind {
            ExpressionKind::Primary(primary) => match primary {
                Primary::Identifier(ref identifier) => {
                    let Some(variable) = self.get_variable(identifier) else {
                        return Err(CompilerError::new(
                            expr.pos.clone(),
                            CompilerErrorKind::UndefinedVariable(identifier.clone()),
                        ));
                    };

                    Ok(variable.typ.clone())
                }
                Primary::Literal(literal) => Ok(match literal {
                    Literal::Int(_) => VariableType::Value(Keyword::Int),
                    Literal::Bool(_) => VariableType::Value(Keyword::Bool),
                    Literal::String(_) => {
                        VariableType::Pointer(Box::new(VariableType::Value(Keyword::Char)))
                    }
                }),
            },
            ExpressionKind::Unary(unary) => {
                let expr_type = self.infer_type(&unary.expr)?;

                match unary.operator {
                    Keyword::Ampersand => Ok(VariableType::Pointer(Box::new(expr_type))),
                    Keyword::Asterix => {
                        let VariableType::Pointer(typ) = expr_type else {
                            return Err(CompilerError::new(
                                unary.operator_pos.clone(),
                                CompilerErrorKind::DereferenceNonPointer(expr_type),
                            ));
                        };

                        Ok(*typ)
                    }
                    _ => todo!("Not supported"),
                }
            }
            ExpressionKind::Binary(binary) => {
                let left = self.infer_type(&binary.left)?;
                let right = self.infer_type(&binary.right)?;

                let Some(typ) = self.infer_binary_result_type(&left, &right, binary.operator) else {
                    return Err(CompilerError::new(
                        binary.left.pos.clone(),
                        CompilerErrorKind::WrongBinaryExpressionTypes {
                            got: left,
                            expected: right,
                            expected_pos: binary.right.pos.clone(),
                            operator: binary.operator,
                            operator_pos: binary.operator_pos.clone(),
                        },
                    ));
                };

                Ok(typ)
            }
            ExpressionKind::FunctionCall(call) => {
                let Some(function) = self.get_variable(&call.identifier) else {
                    return Err(CompilerError::new(
                        call.identifier_pos.clone(),
                        CompilerErrorKind::UndefinedFunction(call.identifier.clone()),
                    ));
                };

                let VariableType::Function(function_id) = function.typ else {
                    todo!("Variable is not a function");
                };

                let Some(return_type) = &self.functions.get(function_id).unwrap().return_type else {
                    todo!("No return type for function");
                };

                Ok(return_type.clone())
            }
            ExpressionKind::StructConstruction(sconst) => {
                Ok(self.get_variable(&sconst.identifier).unwrap().typ.clone())
            }
            ExpressionKind::MemberAccess(access) => self.infer_type(&access.right),
        }
    }
}
