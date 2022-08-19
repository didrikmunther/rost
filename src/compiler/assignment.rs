use crate::{
    lexer::{Keyword, Literal},
    parser::definition::{Assignment, Expression, ExpressionKind, Primary},
};

use super::{
    builder::Builder,
    definition::{Procedure, ProcedureKind},
    error::{CompilerError, CompilerErrorKind},
    program::{Program, Variable},
};

impl Program {
    fn infer_identifier_type(&self, expr: &Expression) -> Result<Keyword, CompilerError> {
        match &expr.kind {
            ExpressionKind::Primary(primary) => match primary {
                Primary::Identifier(ref identifier) => {
                    if let Some(variable) = self.variables.get(identifier) {
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
                let left = self.infer_identifier_type(&binary.left)?;
                let right = self.infer_identifier_type(&binary.right)?;

                if left == right {
                    return Ok(left);
                } else {
                    return Err(CompilerError::new(
                        expr.pos.clone(),
                        CompilerErrorKind::WrongType {
                            got: left,
                            expected: right,
                        },
                    ));
                }
            }
            _ => todo!(),
        }
    }

    pub fn handle_assignment(&mut self, assignment: &Assignment) -> Result<Builder, CompilerError> {
        let expr = self.handle_expression(&assignment.value)?;
        let mut builder = Builder::new().append(expr);

        if let Some(variable) = self.variables.get(&assignment.identifier) {
            if assignment.is_new {
                return Err(CompilerError::new(
                    assignment.identifier_pos.clone(),
                    CompilerErrorKind::RedeclaredVariable(
                        assignment.identifier.clone(),
                        variable.pos.clone(),
                    ),
                ));
            } else {
                let assignment_type = self.infer_identifier_type(&assignment.value)?;

                if assignment_type != variable.typ {
                    return Err(CompilerError::new(
                        assignment.value_pos.clone(),
                        CompilerErrorKind::WrongType {
                            got: assignment_type,
                            expected: variable.typ,
                        },
                    ));
                }

                return Ok(builder.push(Procedure {
                    pos: assignment.identifier_pos.start..assignment.value_pos.end,
                    kind: ProcedureKind::Reassign(variable.stack_pos),
                    comment: Some(format!(
                        "Reassign: {}, stack: {}",
                        assignment.identifier, variable.stack_pos
                    )),
                }));
            }
        } else if !assignment.is_new {
            return Err(CompilerError::new(
                assignment.identifier_pos.clone(),
                CompilerErrorKind::UndefinedVariable(assignment.identifier.clone()),
            ));
        }

        if assignment.is_new {
            if let Some(typ) = assignment.typ {
                self.variables.insert(
                    assignment.identifier.clone(),
                    Variable {
                        pos: assignment.identifier_pos.clone(),
                        typ,
                        stack_pos: self.stack_pos,
                    },
                );
            } else {
                unreachable!()
            }
        }

        builder = builder.push(Procedure::new(
            assignment.identifier_pos.clone(),
            ProcedureKind::Comment(format!(
                "Assignment: {}, stack: {}",
                assignment.identifier.clone(),
                self.stack_pos
            )),
        ));

        self.stack_pos += 1;

        Ok(builder)
    }
}
