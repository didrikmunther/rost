use crate::{
    backend::python::code::Element,
    compiler::program::{
        builder::Builder,
        ir::{FunctionBody, InstructionKind, PrimitiveValue, ProcedureCall, ValueKind, VariableId},
        Program,
    },
};

use super::{code::Code, error::PythonError};

pub struct Generator;

impl Generator {
    pub fn generate_code(self, program: &Program) -> Result<String, PythonError> {
        let root_element = Element::Block(vec![
            Element::Raw(include_str!("boilerplate_entry.py").into()),
            Element::Block(self.get_functions(program)?),
            Element::FunctionDefinition {
                identifier: "__setup".into(),
                content: Box::new(self.get_setup(program)?),
            },
            Element::FunctionDefinition {
                identifier: "__main".into(),
                content: Box::new(Element::Block(vec![
                    self.get_program(&program.instructions, program)?,
                    Element::Pass,
                ])),
            },
            Element::Raw(include_str!("boilerplate_exit.py").into()),
        ]);

        let code = Code::new(root_element);

        Ok(format!("{}", code))
    }

    fn get_user_function_name(&self, variable_id: VariableId) -> String {
        format!("__userf__{}", variable_id)
    }

    pub fn get_functions(&self, program: &Program) -> Result<Vec<Element>, PythonError> {
        let mut elements = vec![];

        for function in &program.functions {
            let var_identifier = &program.variables[function.variable_id].identifier;

            match &function.body {
                FunctionBody::Builtin(builtin_identifier) => {
                    elements.push(Element::Comment(format!(
                        "Builtin function: {builtin_identifier}"
                    )));

                    elements.push(Element::FunctionDefinition {
                        identifier: self.get_user_function_name(function.variable_id),
                        content: Box::new(Element::FunctionCall(format!(
                            "__builtin__{}",
                            var_identifier
                        ))),
                    });
                }
                FunctionBody::Block {
                    body_contains_error: _body_contains_error,
                    content,
                } => {
                    elements.push(Element::Comment(format!(
                        "User function: {}",
                        var_identifier
                    )));

                    elements.push(Element::FunctionDefinition {
                        identifier: format!("__userf__{}", function.variable_id),
                        content: Box::new(Element::Block(vec![
                            self.get_program(content, program)?,
                            Element::Push("0".into()),
                            Element::Pass,
                        ])),
                    });
                }
            }
        }

        Ok(elements)
    }

    pub fn get_setup(&self, program: &Program) -> Result<Element, PythonError> {
        let mut elements: Vec<Element> = vec![
            Element::Raw("global __global_data".into()),
            Element::Raw(format!(
                "__global_data = list(range({}))",
                program.global_data.len()
            )),
        ];

        for (i, global_data) in program.global_data.iter().enumerate() {
            let value = match global_data {
                PrimitiveValue::String(value) => format!("\"{}\"", value),
                PrimitiveValue::Int(value) => value.to_string(),
                _ => todo!(),
            };

            elements.push(Element::Raw(format!("__global_data[{i}] = {value}",)));
        }

        Ok(Element::Block(elements))
    }

    pub fn get_program(
        &self,
        instructions: &Builder,
        program: &Program,
    ) -> Result<Element, PythonError> {
        let mut elements: Vec<Element> = vec![];

        for instruction in instructions.iter() {
            match &instruction.kind {
                InstructionKind::Push(value) => match value {
                    ValueKind::Primitive(primitive) => match primitive {
                        PrimitiveValue::Int(value) => {
                            elements.push(Element::Push(value.to_string()))
                        }
                        PrimitiveValue::String(value) => {
                            elements.push(Element::Push(format!("\"{}\"", value)))
                        }
                        _ => todo!(),
                    },
                    ValueKind::GlobalData(id) => {
                        elements.push(Element::Push(format!("__global_data[{id}]")));
                    }
                    &ValueKind::Variable(id) => {
                        // let variable = &program.variables[id];
                        let variable_name = &program.variables[id].identifier;
                        let variable_name = format!("_{id}_{variable_name}");

                        // let value = match variable.scope {
                        //     VariableScope::Local => variable_name,
                        //     // VariableScope::Global => format!("__global_variables[{variable_name}]"),
                        // };

                        elements.push(Element::Push(variable_name));
                    }
                },
                InstructionKind::Pop => elements.push(Element::Pop),
                InstructionKind::IntAdd => elements.push(Element::Add),
                InstructionKind::IntMul => elements.push(Element::Mul),
                &InstructionKind::Assign(id) => {
                    let variable_name = program.variables[id].identifier.clone();
                    elements.push(Element::Assign(format!("_{id}_{variable_name}")));
                }
                InstructionKind::ProcedureCall(ProcedureCall { variable_id, nargs }) => {
                    elements.push(Element::Push(nargs.to_string()));
                    let identifier = program.variables[*variable_id].identifier.clone();
                    elements.push(Element::Comment(format!("Procedure call: {}", identifier)));
                    elements.push(Element::FunctionCall(
                        self.get_user_function_name(*variable_id),
                    ));
                    elements.push(Element::Pop); // TODO: Currently we don't care about the return value
                }
                _ => {
                    eprintln!("Instruction: {:?}", instruction);
                    todo!()
                }
            }
        }

        Ok(Element::Block(elements))
    }
}
