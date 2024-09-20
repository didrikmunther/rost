use super::{
    builder::Builder,
    ir::{
        Function, FunctionBody, Instruction, InstructionKind, NormalVariable, PrimitiveType,
        Variable, VariableKind, VariableScope,
    },
    Program,
};
use crate::{
    compiler::error::CompilerError,
    parser::definition::{Declaration, FunctionDeclaration, FunctionDeclarationContent},
};

fn add_function_parameters(this: &mut Program, fdec: &FunctionDeclaration) -> Vec<usize> {
    let mut parameter_variable_ids = Vec::new();

    for param in &fdec.parameters {
        let variable_id = this.insert_variable(Variable {
            identifier: param.identifier.clone(),
            scope: VariableScope::Local,
            kind: VariableKind::Normal(NormalVariable {
                typ: PrimitiveType::Int, // TODO: Get the actual type
            }),
            declaration_pos: param.pos.clone(),
            assignment_has_error: false,
        });

        parameter_variable_ids.push(variable_id);
    }

    if let Some(vararg_parameter) = &fdec.vararg_parameter {
        let variable_id = this.insert_variable(Variable {
            identifier: vararg_parameter.identifier.clone(),
            scope: VariableScope::Local,
            kind: VariableKind::Normal(NormalVariable {
                typ: PrimitiveType::Int, // TODO: Get the actual type
            }),
            declaration_pos: vararg_parameter.pos.clone(),
            assignment_has_error: false,
        });

        parameter_variable_ids.push(variable_id);
    }

    parameter_variable_ids
}

impl Program {
    pub fn create_function_declaration(
        &mut self,
        fdec: &FunctionDeclaration,
    ) -> Result<(), CompilerError> {
        let block = match &fdec.content {
            FunctionDeclarationContent::Builtin => {
                let parameter_variable_ids = add_function_parameters(self, fdec);

                self.insert_variable(Variable {
                    identifier: fdec.identifier.clone(),
                    scope: VariableScope::Global,
                    kind: VariableKind::DeclaredFunction(Function {
                        vararg_parameter: parameter_variable_ids.last().copied(),
                        parameter_variable_ids,
                        body: FunctionBody::Builtin,
                    }),
                    declaration_pos: fdec.identifier_pos.clone(),
                    assignment_has_error: false,
                });

                return Ok(());
            }
            FunctionDeclarationContent::Block(block) => block,
        };

        let result = self.with_scope(|this| {
            let location = block.iter().fold(fdec.identifier_pos.clone(), |acc, decl| {
                acc.start..decl.pos.end
            });

            // The first value on the stack on a function call is always amount of arguments.
            // This is used for varargs functions, but we don't have those yet.
            let mut body = Builder::new().push(Instruction::new(
                fdec.identifier_pos.clone(),
                InstructionKind::Pop,
            ));

            let parameter_variable_ids = add_function_parameters(this, fdec);

            for (&variable_id, param) in parameter_variable_ids.iter().zip(fdec.parameters.iter()) {
                body = body.push(Instruction::new(
                    param.pos.clone(),
                    InstructionKind::Assign(variable_id),
                ));
            }

            let body = body.append(this.get_instructions(block)?);

            Ok(((body, parameter_variable_ids), location))
        });

        let body = self.get_or_add_error(result);
        let assignment_has_error = body.is_none();

        let (body, parameter_variable_ids) = body.unwrap_or_default();

        self.insert_variable(Variable {
            identifier: fdec.identifier.clone(),
            scope: VariableScope::Global,
            kind: VariableKind::DeclaredFunction(Function {
                parameter_variable_ids,
                body: FunctionBody::Block(body),
                vararg_parameter: None,
            }),
            declaration_pos: fdec.identifier_pos.clone(),
            assignment_has_error,
        });

        Ok(())
    }

    pub fn handle_function_declaration(
        &mut self,
        _statement: &Declaration,
        fdec: &FunctionDeclaration,
    ) -> Result<Builder, CompilerError> {
        self.create_function_declaration(fdec)?;

        Ok(Builder::new())

        // if let ProgramScope::FunctionScope(_) = &mut self.scope {
        //     todo!("We're already in a function");
        // };

        // let old_stack_pos = self.stack_pos;
        // let return_type = fdec.return_type.as_ref().map(|t| self.get_variable_type(t));

        // let body = self.with_function_scope(return_type.clone(), |this| {
        //     let parameters = fdec
        //         .parameters
        //         .iter()
        //         .rev()
        //         .map(|parameter| {
        //             (
        //                 parameter.identifier.clone(),
        //                 parameter.pos.clone(),
        //                 this.get_variable_type(&parameter.typ),
        //             )
        //         })
        //         .collect::<Vec<_>>();

        //     let ProgramScope::FunctionScope(function_scope) = &mut this.scope else {
        //         unreachable!();
        //     };

        //     for (identifier, pos, typ) in parameters {
        //         function_scope.create_parameter(identifier, Variable { pos, typ });
        //     }

        //     // Calling a function adds the RET address to the stack,
        //     // temporarily compensate for this here.
        //     this.stack_pos += 1;

        //     let procedures = this.get_procedures(&fdec.content)?;

        //     let ProgramScope::FunctionScope(function_scope) = &mut this.scope else {
        //         unreachable!();
        //     };

        //     let variable_sizes = function_scope
        //         .variables
        //         .values()
        //         .map(|variable| Self::get_type_size(&variable.typ))
        //         .sum();

        //     let builder = Builder::new()
        //         .push(Procedure::new(
        //             statement.pos.clone(),
        //             ProcedureKind::Allocate(variable_sizes),
        //         ))
        //         .append(procedures);

        //     Ok(builder)
        // })?;

        // self.functions.push(Function {
        //     body,
        //     parameters: fdec.parameters.clone(),
        //     return_type,
        //     identifier_pos: fdec.identifier_pos.clone(),
        // });

        // let function_location = self.functions.len() - 1;

        // // Pseudo-type-ish variable, does not exist on the stack.
        // self.create_variable(
        //     fdec.identifier.clone(),
        //     Variable {
        //         pos: statement.pos.clone(),
        //         typ: VariableType::Function(function_location),
        //     },
        // );

        // self.stack_pos = old_stack_pos;
    }
}
