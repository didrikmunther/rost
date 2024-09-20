use crate::{
    backend::python::code::Element,
    compiler::program::{
        builder::Builder,
        ir::{
            InstructionKind, PrimitiveValue, ProcedureCall, ValueKind, VariableKind, VariableScope,
        },
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
                content: Box::new(self.get_program(&program.instructions, program)?),
            },
            Element::Raw(include_str!("boilerplate_exit.py").into()),
        ]);

        let code = Code::new(root_element);

        Ok(format!("{}", code))
    }

    pub fn get_functions(&self, program: &Program) -> Result<Vec<Element>, PythonError> {
        let mut elements = vec![];

        let functions = program.variables.iter().filter_map(|variable| {
            if let VariableScope::Global = variable.scope {
                if let VariableKind::DeclaredFunction(function) = &variable.kind {
                    Some((function, variable.identifier.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        });

        for (function, identifier) in functions {
            elements.push(Element::FunctionDefinition {
                identifier: format!("__user__{identifier}"),
                content: Box::new(self.get_program(&function.body, program)?),
            });
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
            match global_data {
                PrimitiveValue::String(value) => {
                    elements.push(Element::Raw(format!("__global_data[{i}] = \"{value}\"",)));
                }
                PrimitiveValue::Int(value) => {
                    elements.push(Element::Raw(format!("__global_data[{i}] = {value}",)));
                }
                _ => todo!(),
            }
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
                        let variable = &program.variables[id];
                        let variable_name = &program.variables[id].identifier;
                        let variable_name = format!("_{id}_{variable_name}");

                        let value = match variable.scope {
                            VariableScope::Local => variable_name,
                            VariableScope::Global => format!("__global_variables[{variable_name}]"),
                        };

                        elements.push(Element::Push(value));
                    }
                },
                InstructionKind::Pop => elements.push(Element::Pop),
                InstructionKind::IntAdd => elements.push(Element::Add),
                InstructionKind::IntMul => elements.push(Element::Mul),
                &InstructionKind::Assign(id) => {
                    let variable_name = program.variables[id].identifier.clone();
                    elements.push(Element::Assign(format!("_{id}_{variable_name}")));
                }
                InstructionKind::SystemCall(ProcedureCall { variable_id, nargs }) => {
                    elements.push(Element::Push(nargs.to_string()));
                    let identifier = program.variables[*variable_id].identifier.clone();
                    elements.push(Element::FunctionCall(format!("__builtin__{identifier}")))
                }
                InstructionKind::ProcedureCall(ProcedureCall { variable_id, nargs }) => {
                    elements.push(Element::Push(nargs.to_string()));
                    let identifier = program.variables[*variable_id].identifier.clone();
                    elements.push(Element::FunctionCall(format!("__user__{identifier}")))
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
