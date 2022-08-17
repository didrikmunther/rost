use crate::{
    lexer::Literal,
    parser::definition::{ExpressionKind, FunctionCall, Primary},
};

use super::{
    definition::{GlobalData, RegisterValue, SystemCall},
    error::{CompilerError, CompilerErrorKind},
    program::Program,
};

impl Program {
    pub fn handle_function_call(
        &mut self,
        fcall: &FunctionCall,
    ) -> Result<SystemCall, CompilerError> {
        let mut args: Vec<RegisterValue> = Vec::new();

        for arg in &fcall.args {
            match &arg.kind {
                ExpressionKind::Primary(primary) => match primary {
                    Primary::Literal(literal) => match literal {
                        Literal::String(s) => {
                            self.global_data.push(GlobalData { content: s.clone() });

                            // push latest index
                            args.push(RegisterValue::ByteLocation(self.global_data.len() - 1));
                        }
                        Literal::Int(i) => {
                            args.push(RegisterValue::Int(*i));
                        }
                        _ => {}
                    },
                    _ => {
                        return Err(CompilerError::new(
                            arg.pos.clone(),
                            CompilerErrorKind::Unimplemented,
                        ))
                    }
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
