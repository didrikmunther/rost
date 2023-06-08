use crate::parser::definition::{Declaration, FunctionDeclaration};

use super::{
    builder::Builder,
    definition::{Function, Procedure, ProcedureKind},
    error::CompilerError,
    program::Program,
    scope::{
        variable::{Variable, VariableType},
        ProgramScope,
    },
};

impl Program {
    pub fn handle_function_declaration(
        &mut self,
        statement: &Declaration,
        fdec: &FunctionDeclaration,
    ) -> Result<Builder, CompilerError> {
        if let ProgramScope::FunctionScope(_) = &mut self.scope {
            todo!("We're already in a function");
        };

        let old_stack_pos = self.stack_pos;
        let return_type = fdec.return_type.as_ref().map(|t| self.get_variable_type(t));

        let body = self.with_function_scope(return_type.clone(), |this| {
            let parameters = fdec
                .parameters
                .iter()
                .rev()
                .map(|parameter| {
                    (
                        parameter.identifier.clone(),
                        parameter.pos.clone(),
                        this.get_variable_type(&parameter.typ),
                    )
                })
                .collect::<Vec<_>>();

            let ProgramScope::FunctionScope(function_scope) = &mut this.scope else {
                unreachable!();
            };

            for (identifier, pos, typ) in parameters {
                function_scope.create_parameter(identifier, Variable { pos, typ });
            }

            // Calling a function adds the RET address to the stack,
            // temporarily compensate for this here.
            this.stack_pos += 1;

            let procedures = this.get_procedures(&fdec.content)?;

            let ProgramScope::FunctionScope(function_scope) = &mut this.scope else {
                unreachable!();
            };

            let variable_sizes = function_scope
                .variables
                .values()
                .map(|variable| Self::get_type_size(&variable.typ))
                .sum();

            let builder = Builder::new()
                .push(Procedure::new(
                    statement.pos.clone(),
                    ProcedureKind::Allocate(variable_sizes),
                ))
                .append(procedures);

            Ok(builder)
        })?;

        self.functions.push(Function {
            body,
            parameters: fdec.parameters.clone(),
            return_type,
        });

        let function_location = self.functions.len() - 1;

        // Pseudo-type-ish variable, does not exist on the stack.
        self.create_variable(
            fdec.identifier.clone(),
            Variable {
                pos: statement.pos.clone(),
                typ: VariableType::Function(function_location),
            },
        );

        self.stack_pos = old_stack_pos;

        Ok(Builder::new())
    }
}
