use crate::parser::definition::{Declaration, FunctionDeclaration};

use super::{
    builder::Builder,
    definition::{Function, OperandValue, Procedure, ProcedureKind},
    error::CompilerError,
    program::Program,
    scope::{Variable, VariableType},
};

impl Program {
    pub fn handle_function_declaration(
        &mut self,
        statement: &Declaration,
        fdec: &FunctionDeclaration,
    ) -> Result<Builder, CompilerError> {
        let npars = fdec.parameters.len();
        let old_stack_pos = self.stack_pos;
        let return_type = fdec.return_type.clone();

        let body = self.with_function_scope(return_type.clone(), |this| {
            for parameter in fdec.parameters.iter() {
                this.create_parameter(
                    parameter.identifier.clone(),
                    Variable {
                        pos: parameter.pos.clone(),
                        typ: VariableType::Value(parameter.typ),
                    },
                );

                // this.stack_pos += 1;
            }

            // Calling a function adds the RET address to the stack,
            // temporarily compensate for this here.
            this.stack_pos += 1;

            let procedures = this.get_procedures(&fdec.content)?;
            let nvars = this.function_scope.variables.len();

            let builder = Builder::new()
                .push(Procedure::new(
                    statement.pos.clone(),
                    ProcedureKind::Allocate(nvars),
                ))
                .append(procedures);

            Ok(builder)
        })?;

        self.functions.push(Function {
            body,
            npars,
            return_type,
        });

        let function_location = self.functions.len() - 1;

        self.create_variable(
            fdec.identifier.clone(),
            Variable {
                pos: statement.pos.clone(),
                typ: VariableType::Function(function_location),
            },
        );

        self.stack_pos = old_stack_pos;

        // let builder = Builder::new().push(Procedure::new(
        //     statement.pos.clone(),
        //     ProcedureKind::Push(OperandValue::FunctionLocation(function_location)),
        // ));

        Ok(Builder::new())
    }
}
