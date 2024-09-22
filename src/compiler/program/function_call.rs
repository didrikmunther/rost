use crate::{
    compiler::{
        error::{CompilerError, CompilerErrorKind},
        program::ir::{Instruction, InstructionKind, ProcedureCall},
    },
    parser::definition::{
        Expression, FunctionCall, FunctionDeclaration, FunctionDeclarationContent,
    },
};

use super::{
    builder::Builder,
    ir::{ExpressedType, Function, FunctionBody, TypeKind, Variable, VariableId},
    Program,
};

fn add_function_parameters(this: &mut Program, fdec: &FunctionDeclaration) -> Vec<VariableId> {
    let mut parameter_variable_ids = Vec::new();

    for param in &fdec.parameters {
        let picked_typ = this.get_declared_type(&param.typ).unwrap();

        let variable_id = this.insert_variable(
            Variable {
                identifier: param.identifier.clone(),
                typ: picked_typ,
                declaration_pos: param.pos.clone(),
                assignment_has_error: false,
            },
            Some(param.identifier.as_str()),
        );

        parameter_variable_ids.push(variable_id);
    }

    if let Some(vararg_parameter) = &fdec.vararg_parameter {
        let picked_typ = this.get_declared_type(&vararg_parameter.typ).unwrap();

        let variable_id = this.insert_variable(
            Variable {
                identifier: vararg_parameter.identifier.clone(),
                typ: picked_typ,
                declaration_pos: vararg_parameter.pos.clone(),
                assignment_has_error: false,
            },
            Some(vararg_parameter.identifier.as_str()),
        );

        parameter_variable_ids.push(variable_id);
    }

    parameter_variable_ids
}

impl Program {
    pub fn handle_function_call(
        &mut self,
        expression: &Expression,
        fcall: &FunctionCall,
    ) -> Result<Builder, CompilerError> {
        let mut builder = Builder::new();

        for arg in fcall.args.iter().rev() {
            let result = self.handle_expression(arg);
            if let Some(expr) = self.get_or_add_error(result) {
                builder = builder.append(expr);
            }
        }

        let identifier = fcall.left.get_string().unwrap().to_string();

        let Some(function_template_id) = self
            .get_scope()
            .function_template_lookup
            .get(&identifier)
            .copied()
        else {
            return Err(CompilerError::new(
                fcall.left.pos.clone(),
                CompilerErrorKind::UndefinedFunction(identifier),
            ));
        };

        // let function_type = self
        //     .function_templates
        //     .get(function_template_id)
        //     .and_then(|function_template| self.types.get(function_template.typ))
        //     .unwrap();

        let function_template = self.function_templates.get(function_template_id).unwrap();
        let function_template_typ = function_template.typ;
        let function_type = self.types.get(function_template.typ).unwrap();

        let TypeKind::Function {
            parameter_type_ids,
            vararg_parameter_type_id,
            declaration_pos,
        } = &function_type.kind
        else {
            return Err(CompilerError::new(
                fcall.left.pos.clone(),
                CompilerErrorKind::NotAFunction(identifier),
            ));
        };

        let parameter_types = parameter_type_ids
            .iter()
            .map(|(identifier, id)| (identifier, &self.types[*id]))
            .collect::<Vec<_>>();

        let vararg_parameter_type = vararg_parameter_type_id
            .as_ref()
            .map(|(identifier, id)| (identifier, self.types.get(*id).unwrap()));

        let parameter_diff = parameter_types.len() as i32 - fcall.args.len() as i32;

        match parameter_diff {
            0 => {}
            _ if parameter_diff > 0 => {
                self.add_error(CompilerError::new(
                    fcall.left.pos.clone(),
                    CompilerErrorKind::NotEnoughFunctionArguments {
                        got: fcall.args.len(),
                        missing: parameter_types[parameter_types.len() - parameter_diff as usize..]
                            .iter()
                            .map(|&(identifier, _typ)| identifier.clone())
                            .collect(),
                    },
                ));
            }
            _ if vararg_parameter_type.is_some() && parameter_diff < 0 => {}
            _ if parameter_diff < 0 => {
                let additional = fcall.args.get(parameter_types.len()..).unwrap();
                let pos = additional
                    .iter()
                    .map(|arg| arg.pos.clone())
                    .fold(additional.first().unwrap().pos.clone(), |acc, pos| {
                        acc.start..pos.end
                    });

                self.add_error(CompilerError::new(
                    pos,
                    CompilerErrorKind::TooManyFunctionArguments {
                        expected: parameter_types.len(),
                        got: fcall.args.len(),
                    },
                ));
            }
            _ => unreachable!(),
        }

        let fdec = self
            .function_templates
            .get(function_template_id)
            .unwrap()
            .function_declaration
            .clone();

        let result = self.with_scope(|this| {
            match &fdec.content {
                FunctionDeclarationContent::Block(block) => {
                    let location = block.iter().fold(fdec.identifier_pos.clone(), |acc, decl| {
                        acc.start..decl.pos.end
                    });

                    // The first value on the stack on a function call is always amount of arguments.
                    // This is used for varargs functions, but we don't have those yet.
                    let mut body = Builder::new().push(Instruction::new(
                        fdec.identifier_pos.clone(),
                        InstructionKind::Pop,
                    ));

                    let parameter_variable_ids = add_function_parameters(this, &fdec);

                    for (&variable_id, param) in
                        parameter_variable_ids.iter().zip(fdec.parameters.iter())
                    {
                        body = body.push(Instruction::new(
                            param.pos.clone(),
                            InstructionKind::Assign(variable_id),
                        ));
                    }

                    body = body.append(this.get_instructions(block)?);

                    Ok((
                        FunctionBody::Block {
                            body_contains_error: false,
                            content: body,
                        },
                        location,
                    ))
                }
                FunctionDeclarationContent::Builtin => Ok((
                    FunctionBody::Builtin(fdec.identifier.clone()),
                    fdec.identifier_pos.clone(),
                )),
            }
        });

        let body = self
            .get_or_add_error(result)
            .unwrap_or(FunctionBody::Block {
                body_contains_error: true,
                content: Builder::new(),
            });

        let variable_id = self.insert_variable(
            Variable {
                identifier: identifier.clone(),
                typ: ExpressedType {
                    id: function_template_typ,
                    arguments: None,
                },
                declaration_pos: fdec.identifier_pos.clone(),
                assignment_has_error: false,
            },
            Some(identifier.as_str()),
        );

        self.insert_function(Function {
            function_template_id,
            variable_id,
            parameter_variable_ids: vec![],
            vararg_parameter: None,
            body,
        });

        // for ((parameter_identifier, parameter), arg) in parameter_types.iter().zip(fcall.args.iter()) {
        //     let arg_type = self.infer_type(arg)?;
        //     if arg_type != parameter {
        //         self.add_error(CompilerError::new(
        //             arg.pos.clone(),
        //             CompilerErrorKind::InvalidFunctionArgumentType {
        //                 expected: parameter.1.clone(),
        //                 got: arg_type,
        //             },
        //         ));
        //     }
        // }

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
