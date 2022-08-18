use crate::{
    lexer::Literal,
    parser::definition::{ExpressionKind, FunctionCall, Primary},
};

use super::{
    definition::{GlobalData, OperandValue, SystemCall},
    error::{CompilerError, CompilerErrorKind},
    program::Program,
};

impl Program {
    pub fn handle_function_call(
        &mut self,
        fcall: &FunctionCall,
    ) -> Result<SystemCall, CompilerError> {
        let mut args: Vec<OperandValue> = Vec::new();

        for arg in &fcall.args {
            match &arg.kind {
                ExpressionKind::Primary(primary) => match primary {
                    Primary::Literal(literal) => match literal {
                        Literal::String(s) => {
                            self.global_data.push(GlobalData { content: s.clone() });

                            // push latest index
                            args.push(OperandValue::ByteLocation(self.global_data.len() - 1));
                        }
                        Literal::Int(i) => {
                            args.push(OperandValue::Int(*i));
                        }
                        _ => {}
                    },
                    Primary::Identifier(identifier) => {
                        if let Some(location) = self.variables.get(identifier) {
                            args.push(OperandValue::StackLocation(*location));
                        } else {
                            return Err(CompilerError::new(
                                arg.pos.clone(),
                                CompilerErrorKind::UndefinedVariable(identifier.clone()),
                            ));
                        }
                    }
                    // _ => {
                    //     return Err(CompilerError::new(
                    //         arg.pos.clone(),
                    //         CompilerErrorKind::Unimplemented,
                    //     ))
                    // }
                },
                _ => {
                    return Err(CompilerError::new(
                        arg.pos.clone(),
                        CompilerErrorKind::Unimplemented,
                    ))
                }
            }
        }

        let identifier = match &fcall.identifier.kind {
            ExpressionKind::Primary(primary) => match primary {
                Primary::Identifier(s) => s.clone(),
                _ => {
                    return Err(CompilerError::new(
                        fcall.identifier.pos.clone(),
                        CompilerErrorKind::Unimplemented,
                    ))
                }
            },
            _ => {
                return Err(CompilerError::new(
                    fcall.identifier.pos.clone(),
                    CompilerErrorKind::Unimplemented,
                ))
            }
        };

        Ok(SystemCall { identifier, args })
    }
}
