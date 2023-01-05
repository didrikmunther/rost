use crate::{
    lexer::{Keyword, Literal},
    parser::definition::{Expression, ExpressionKind, Primary},
};

use super::{
    error::{CompilerError, CompilerErrorKind},
    program::Program,
    scope::Variable,
};

impl<'a> Program {
    pub fn get_variable(&'a self, identifier: &String) -> Option<&'a Variable> {
        self.scope.get_variable(identifier)
    }

    pub fn insert_variable(&'a mut self, identifier: String, variable: Variable) {
        self.scope.insert_variable(identifier, variable)
    }

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
                        Ok(variable.typ)
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
            _ => todo!(),
        }
    }
}
