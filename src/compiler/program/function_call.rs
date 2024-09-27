use std::{collections::HashMap, ops::Range};

use crate::{
    compiler::{
        error::{
            CompilerError, CompilerErrorKind, WrongFunctionArguments, WrongGenericFunctionArguments,
        },
        program::{
            ir::{Instruction, InstructionKind, ProcedureCall},
            scope::ScopedVariable,
        },
    },
    parser::definition::{
        Expression, FunctionCall, FunctionDeclaration, FunctionDeclarationContent,
    },
};

use super::{
    builder::Builder,
    ir::{Function, FunctionBody, FunctionId, Variable, VariableId},
    typ::{ExpressedType, FunctionTypeKind, TypeId, TypeIdWithIdentifier, TypeKind},
    Program,
};

pub enum FunctionCreationError {
    NoSuchFunction,
}

fn add_function_parameters(this: &mut Program, fdec: &FunctionDeclaration) -> Vec<VariableId> {
    let mut parameter_variable_ids = Vec::new();

    for param in &fdec.parameters {
        let picked_typ = this.get_declared_type(&param.typ).unwrap();

        let variable_id = this.insert_argument(
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
    fn insert_function(&mut self, function: Function) -> FunctionId {
        let function_id = self.functions.len();
        self.functions.push(function);

        function_id
    }

    fn check_parameter_correctness(
        &mut self,
        function_type_kind: &FunctionTypeKind,
        function_template: &FunctionDeclaration,
        fcall: &FunctionCall,
    ) -> Result<(), CompilerError> {
        let vararg_parameter_type = function_type_kind
            .vararg_parameter_type_id
            .as_ref()
            .map(|TypeIdWithIdentifier { identifier, id }| {
                (identifier, id, self.types.get(*id).unwrap())
            })
            .map(|(a, b, c)| (a.clone(), *b, c.clone()));

        let parameter_types = function_type_kind
            .parameter_type_ids
            .iter()
            .cloned()
            .map(|TypeIdWithIdentifier { id, identifier }| (identifier, id, self.types[id].clone()))
            .collect::<Vec<_>>();

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
                            .map(|(identifier, _id, _typ)| identifier.clone())
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

        let par_args = parameter_types
            .iter()
            .map(Some)
            .chain(std::iter::repeat(None))
            .zip(fcall.args.iter().map(Some).chain(std::iter::repeat(None)))
            .take_while(|(x, y)| x.is_some() || y.is_some())
            .collect::<Vec<_>>();

        let mut generic_to_type_map = HashMap::<String, (TypeId, Range<usize>)>::new();

        for ((mut par, arg), mut fdec_par) in par_args
            .into_iter()
            .zip(function_template.parameters.iter())
        {
            if par.is_none() {
                par = vararg_parameter_type.as_ref();
                fdec_par = function_template.vararg_parameter.as_ref().unwrap();
            }

            if let Some(arg) = arg {
                let arg_typ = self.infer_type(arg)?;
                let is_any = match &par.unwrap().2.kind {
                    TypeKind::Intrinsic { identifier } => identifier == "any",
                    _ => false,
                };
                let is_unknown = match &self.types.get(arg_typ.id).unwrap().kind {
                    TypeKind::Intrinsic { identifier } => identifier == "unknown",
                    _ => false,
                };

                let par_id = match &par.unwrap().2.kind {
                    TypeKind::Generic {
                        identifier,
                        declaration_pos,
                    } => {
                        if let Some((id, generic_declaration_pos)) =
                            generic_to_type_map.get(identifier)
                        {
                            if !is_unknown && arg_typ.id != *id {
                                self.add_error(
                                    CompilerErrorKind::WrongGenericFunctionArguments(
                                        WrongGenericFunctionArguments::from_expressed_type(
                                            identifier.clone(),
                                            declaration_pos.clone(),
                                            generic_declaration_pos.clone(),
                                            *id,
                                            arg_typ.id,
                                            fdec_par.pos.clone(),
                                            self,
                                        ),
                                    )
                                    .at_pos(&arg.pos),
                                );

                                None
                            } else {
                                Some(*id)
                            }
                        } else {
                            generic_to_type_map
                                .insert(identifier.clone(), (arg_typ.id, arg.pos.clone()));
                            Some(arg_typ.id)
                        }
                    }
                    _ => Some(par.unwrap().1),
                };

                if let Some(par_id) = par_id {
                    if !is_any && !is_unknown && arg_typ.id != par_id {
                        self.add_error(
                            CompilerErrorKind::WrongFunctionArguments(
                                WrongFunctionArguments::from_expressed_type(
                                    par_id,
                                    arg_typ.id,
                                    fdec_par.pos.clone(),
                                    self,
                                ),
                            )
                            .at_pos(&arg.pos),
                        );
                    }
                }
            }
        }

        Ok(())
    }

    pub fn create_function(
        &mut self,
        identifier: &str,
    ) -> Result<VariableId, FunctionCreationError> {
        let Some(function_template_id) = self
            .get_scope()
            .function_template_lookup
            .get(identifier)
            .copied()
        else {
            return Err(FunctionCreationError::NoSuchFunction);
        };

        let function_template = self.function_templates.get(function_template_id).unwrap();
        let function_template_typ = function_template.typ;

        let fdec = self
            .function_templates
            .get(function_template_id)
            .unwrap()
            .function_declaration
            .clone();

        let result = self.with_scope(|this| match &fdec.content {
            FunctionDeclarationContent::Block(block) => {
                let location = block.iter().fold(fdec.identifier_pos.clone(), |acc, decl| {
                    acc.start..decl.pos.end
                });

                let mut body = Builder::new();

                let parameter_variable_ids = add_function_parameters(this, &fdec);
                let instructions = this.get_instructions(block);
                let function_body = this.get_or_add_error(instructions);
                let body_contains_error = function_body.is_none();
                body = body.append(function_body.unwrap_or_default());

                let declared_variables: Vec<VariableId> = this
                    .get_scope()
                    .variable_lookup
                    .values()
                    .filter_map(|v| match v {
                        ScopedVariable::Native(v) => Some(*v),
                        _ => None,
                    })
                    .collect();

                Ok((
                    (
                        FunctionBody::Block {
                            body_contains_error,
                            content: body,
                        },
                        parameter_variable_ids,
                        declared_variables,
                    ),
                    location,
                ))
            }
            FunctionDeclarationContent::Builtin => Ok((
                (
                    FunctionBody::Builtin(fdec.identifier.clone()),
                    Vec::new(),
                    Vec::new(),
                ),
                fdec.identifier_pos.clone(),
            )),
        });

        let (function_body, parameter_variable_ids, declared_variables) =
            self.get_or_add_error(result).unwrap_or((
                FunctionBody::Block {
                    body_contains_error: true,
                    content: Builder::new(),
                },
                Vec::new(),
                Vec::new(),
            ));

        let variable_id = self.insert_variable(
            Variable {
                identifier: identifier.to_string(),
                typ: ExpressedType {
                    id: function_template_typ,
                    arguments: None,
                },
                declaration_pos: fdec.identifier_pos.clone(),
                assignment_has_error: false,
            },
            Some(identifier),
        );

        self.insert_function(Function {
            function_template_id,
            variable_id,
            parameter_variable_ids,
            vararg_parameter: None,
            body: function_body,
            declared_variables,
        });

        Ok(variable_id)
    }

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
            return CompilerErrorKind::UndefinedFunction(identifier)
                .at_pos(&fcall.left.pos)
                .into();
        };

        let function_template = self.function_templates.get(function_template_id).unwrap();
        let function_type = self.types.get(function_template.typ).unwrap();

        let TypeKind::Function(function_type_kind) = &function_type.kind.clone() else {
            return CompilerErrorKind::NotAFunction(identifier)
                .at_pos(&fcall.left.pos)
                .into();
        };

        self.check_parameter_correctness(
            function_type_kind,
            &function_template.function_declaration.clone(),
            fcall,
        )?;

        let variable_id = match self.create_function(&identifier) {
            Ok(variable_id) => variable_id,
            Err(FunctionCreationError::NoSuchFunction) => {
                return CompilerErrorKind::UndefinedFunction(identifier)
                    .at_pos(&fcall.left.pos)
                    .into();
            }
        };

        // Backends should handle return values. E.g. are they on stack, or in a register?
        let builder = builder.push(Instruction::new(
            expression.pos.clone(),
            InstructionKind::ProcedureCall(ProcedureCall {
                nargs: fcall.args.len(),
                variable_id,
            }),
        ));

        Ok(builder)
    }
}
