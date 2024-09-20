use crate::{
    compiler::{
        error::{CompilerError, CompilerErrorKind},
        program::ir::{Instruction, InstructionKind, ProcedureCall},
    },
    parser::definition::{Expression, FunctionCall},
};

use super::{builder::Builder, Program};

static BUILT_IN: &[&str] = &["printf"];

impl Program {
    pub fn handle_function_call(
        &mut self,
        expression: &Expression,
        fcall: &FunctionCall,
    ) -> Result<Builder, CompilerError> {
        let mut builder = Builder::new();

        for arg in &fcall.args {
            let expr = self.handle_expression(arg)?;
            builder = builder.append(expr);
        }

        let identifier = fcall.left.get_string().unwrap().to_string();

        // if BUILT_IN.contains(&identifier.as_str()) {
        //     return Ok(builder
        //         .push(Instruction::new(
        //             expression.pos.clone(),
        //             InstructionKind::SystemCall(ProcedureCall {
        //                 nargs: fcall.args.len(),
        //                 identifier,
        //             }),
        //         ))
        //         .push(Instruction::new(
        //             expression.pos.clone(),
        //             InstructionKind::Pop,
        //         )));
        // }

        let Some(variable_id) = self.get_variable(&identifier) else {
            return Err(CompilerError::new(
                fcall.left.pos.clone(),
                CompilerErrorKind::UndefinedFunction(identifier),
            ));
        };

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

        // let Some(variable) = self.get_variable(&identifier) else {
        //     return Err(CompilerError::new(
        //         fcall.left.pos.clone(),
        //         CompilerErrorKind::UndefinedFunction(identifier),
        //     ));
        // };

        // let VariableType::Function(function_id) = variable.typ else {
        //     todo!("Variable is not a function")
        // };

        // let function = self.functions.get(function_id).unwrap();

        // if function.parameters.len() != fcall.args.len() {
        //     todo!(
        //         "Wrong number of arguments to function, takes {}, {} was given",
        //         function.parameters.len(),
        //         fcall.args.len()
        //     )
        // }

        // for (par, arg) in function.parameters.iter().zip(&fcall.args) {
        //     let arg_type = self.infer_type(arg)?;
        //     let par_type = self.get_variable_type(&par.typ);

        //     if arg_type != par_type {
        //         return Err(CompilerError::new(
        //             arg.pos.clone(),
        //             CompilerErrorKind::WrongArgumentType {
        //                 parameter: par_type,
        //                 argument: arg_type,
        //                 parameter_pos: par.pos.clone(),
        //             },
        //         ));
        //     }
        // }

        // builder = builder.push(Procedure::new(
        //     expression.pos.clone(),
        //     ProcedureKind::ProcedureCall(ProcedureCall {
        //         function_id,
        //         nargs: fcall.args.len(),
        //         returns: function.return_type.is_some(),
        //     }),
        // ));

        // Ok(builder)
    }
}
