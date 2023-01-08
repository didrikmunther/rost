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
    pub fn get_procedures(&mut self, content: &Vec<Declaration>) -> Result<Builder, CompilerError> {
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

    pub fn infer_binary_result_type(
        &self,
        left: &VariableType,
        right: &VariableType,
        operator: Keyword,
    ) -> Option<Keyword> {
        // if let VariableType::Pointer(left) = left {

        // }

        let (VariableType::Value(left), VariableType::Value(right)) = (left, right) else {
            return None;
        };

        match operator {
            Keyword::Plus | Keyword::Minus | Keyword::Slash | Keyword::Asterix => {
                if left == right {
                    Some(left.clone())
                } else {
                    None
                }
            }
            Keyword::LessThan | Keyword::GreaterThan | Keyword::Equality => {
                if left == right {
                    Some(Keyword::Bool)
                } else {
                    None
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
        }
    }

    pub fn infer_type(&self, expr: &Expression) -> Result<VariableType, CompilerError> {
        match &expr.kind {
            ExpressionKind::Primary(primary) => match primary {
                Primary::Identifier(ref identifier) => {
                    if let Some(variable) = self.get_variable(identifier) {
                        Ok(variable.typ.clone())
                    } else {
                        return Err(CompilerError::new(
                            expr.pos.clone(),
                            CompilerErrorKind::UndefinedVariable(identifier.clone()),
                        ));
                    }
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
                    Keyword::Ampersand => return Ok(VariableType::Pointer(Box::new(expr_type))),
                    _ => todo!("Not supported"),
                }
            }
            ExpressionKind::Binary(binary) => {
                let left = self.infer_type(&binary.left)?;
                let right = self.infer_type(&binary.right)?;

                if let Some(typ) = self.infer_binary_result_type(&left, &right, binary.operator) {
                    return Ok(VariableType::Value(typ));
                } else {
                    println!("expected: {:?}, {:?}", binary.left.pos, binary.right.pos);

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
                }
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
        }
    }
}
