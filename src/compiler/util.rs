use crate::{
    lexer::{Keyword, Literal},
    parser::definition::{Declaration, Expression, ExpressionKind, Primary},
};

use super::{
    builder::Builder,
    error::{CompilerError, CompilerErrorKind},
    program::Program,
    scope::{
        variable::{StoredVariable, Variable, VariableType},
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

    /// Creates a stack allocated variable.
    /// Returns stack position of the stack allocated variable.
    pub fn create_variable(&mut self, identifier: String, variable: Variable) -> usize {
        match &mut self.scope {
            ProgramScope::RootScope(scope) => scope.create_variable(identifier, variable),
            ProgramScope::FunctionScope(scope) => scope.create_variable(identifier, variable),
        }
    }

    // /// Creates a stack allocated parameter to function.
    // /// Returns stack position of the stack allocated variable.
    // pub fn create_parameter(&mut self, identifier: String, variable: Variable) -> isize {
    //     self.function_scope.create_parameter(identifier, variable)
    // }

    pub fn infer_binary_result_type(
        &self,
        left: Keyword,
        right: Keyword,
        operator: Keyword,
    ) -> Option<Keyword> {
        match operator {
            Keyword::Plus | Keyword::Minus | Keyword::Slash | Keyword::Asterix => {
                if left == right {
                    Some(left)
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

    pub fn infer_type(&self, expr: &Expression) -> Result<Keyword, CompilerError> {
        match &expr.kind {
            ExpressionKind::Primary(primary) => match primary {
                Primary::Identifier(ref identifier) => {
                    if let Some(variable) = self.get_variable(identifier) {
                        Ok(variable.typ.to_keyword())
                    } else {
                        return Err(CompilerError::new(
                            expr.pos.clone(),
                            CompilerErrorKind::UndefinedVariable(identifier.clone()),
                        ));
                    }
                }
                Primary::Literal(literal) => Ok(match literal {
                    Literal::Int(_) => Keyword::Int,
                    Literal::String(_) => Keyword::String,
                    Literal::Bool(_) => Keyword::Bool,
                }),
            },
            ExpressionKind::Binary(binary) => {
                let left = self.infer_type(&binary.left)?;
                let right = self.infer_type(&binary.right)?;

                if let Some(typ) = self.infer_binary_result_type(left, right, binary.operator) {
                    return Ok(typ);
                } else {
                    return Err(CompilerError::new(
                        binary.left.pos.clone(),
                        CompilerErrorKind::WrongBinaryExpressionTypes {
                            got: left,
                            expected: right,
                            expected_pos: binary.right.pos.clone(),
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

                return Ok(self
                    .functions
                    .get(function_id)
                    .unwrap()
                    .return_type
                    .get_keyword_type());
            }
        }
    }
}
