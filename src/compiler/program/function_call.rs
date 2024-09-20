use crate::{
    compiler::{
        error::{CompilerError, CompilerErrorKind},
        program::ir::{Instruction, InstructionKind, ProcedureCall},
    },
    parser::definition::{Expression, FunctionCall},
};

use super::{builder::Builder, ir::VariableKind, Program};

impl Program {
    pub fn handle_function_call(
        &mut self,
        expression: &Expression,
        fcall: &FunctionCall,
    ) -> Result<Builder, CompilerError> {
        let mut builder = Builder::new();

        for arg in &fcall.args {
            let result = self.handle_expression(arg);
            if let Some(expr) = self.get_or_add_error(result) {
                builder = builder.append(expr);
            }
        }

        let identifier = fcall.left.get_string().unwrap().to_string();

        let Some((variable_id, variable)) = self
            .get_variable(&identifier)
            .map(|variable_id| (variable_id, &self.variables[variable_id]))
        else {
            return Err(CompilerError::new(
                fcall.left.pos.clone(),
                CompilerErrorKind::UndefinedFunction(identifier),
            ));
        };

        let VariableKind::DeclaredFunction(function) = &variable.kind else {
            return Err(CompilerError::new(
                fcall.left.pos.clone(),
                CompilerErrorKind::NotAFunction(identifier),
            ));
        };

        let parameters = function
            .parameter_variable_ids
            .iter()
            .map(|&id| &self.variables[id])
            .collect::<Vec<_>>();

        let parameter_diff = parameters.len() as i32 - fcall.args.len() as i32;

        match parameter_diff {
            0 => {}
            _ if parameter_diff > 0 => {
                return Err(CompilerError::new(
                    fcall.left.pos.clone(),
                    CompilerErrorKind::NotEnoughFunctionArguments {
                        got: fcall.args.len(),
                        missing: parameters[parameters.len() - parameter_diff as usize..]
                            .iter()
                            .map(|v| v.identifier.clone())
                            .collect(),
                    },
                ));
            }
            _ if parameter_diff < 0 => {
                return Err(CompilerError::new(
                    fcall.left.pos.clone(),
                    CompilerErrorKind::TooManyFunctionArguments {
                        expected: parameters.len(),
                        got: fcall.args.len(),
                    },
                ));
            }
            _ => unreachable!(),
        }

        let builder = builder
            .push(Instruction::new(
                expression.pos.clone(),
                InstructionKind::ProcedureCall(ProcedureCall {
                    nargs: fcall.args.len(),
                    variable_id,
                }),
            ))
            .push(Instruction::new(
                expression.pos.clone(),
                InstructionKind::Pop,
            ));

        Ok(builder)
    }
}
