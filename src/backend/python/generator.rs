use crate::{
    backend::python::code::Element,
    compiler::program::{
        ir::{InstructionKind, PrimitiveValue, SystemCall, ValueKind},
        Program,
    },
};

use super::{code::Code, error::PythonError};

pub struct Generator<'a> {
    pub program: &'a Program,
}

impl<'a> Generator<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self { program }
    }

    pub fn generate_code(self) -> Result<String, PythonError> {
        let root_element = Element::Block(vec![
            Element::Raw(include_str!("boilerplate_entry.py").into()),
            Element::FunctionDefinition {
                name: "__setup".into(),
                params: vec![],
                content: Box::new(self.get_setup()?),
            },
            Element::FunctionDefinition {
                name: "__main".into(),
                params: vec!["argc".into(), "argv".into()],
                content: Box::new(self.get_program()?),
            },
            Element::Raw(include_str!("boilerplate_exit.py").into()),
        ]);

        let code = Code::new(root_element);

        Ok(format!("{}", code))
    }

    pub fn get_setup(&self) -> Result<Element, PythonError> {
        let mut elements: Vec<Element> = vec![
            Element::Raw("global __global_data".into()),
            Element::Raw(format!(
                "__global_data = list(range({}))",
                self.program.global_data.len()
            )),
        ];

        for (i, global_data) in self.program.global_data.iter().enumerate() {
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

    pub fn get_program(&self) -> Result<Element, PythonError> {
        let mut elements: Vec<Element> = vec![];

        for instruction in self.program.instructions.iter() {
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
                        let variable_name = self.program.variables[id].name.clone();
                        elements.push(Element::Raw(format!(
                            "__intrinsic__stack_push(_{id}_{variable_name})"
                        )));
                    }
                },
                InstructionKind::Pop => {
                    elements.push(Element::Raw("__intrinsic__stack_pop()".into()))
                }
                InstructionKind::IntAdd => {
                    elements.push(Element::Raw("__intrinsic__stack_add()".into()))
                }
                InstructionKind::IntMul => {
                    elements.push(Element::Raw("__intrinsic__stack_mul()".into()))
                }
                &InstructionKind::Assign(id) => {
                    let variable_name = self.program.variables[id].name.clone();
                    elements.push(Element::Raw(format!(
                        "_{id}_{variable_name} = __intrinsic__stack_pop()"
                    )));
                }
                InstructionKind::SystemCall(SystemCall { identifier, nargs }) => {
                    elements.push(Element::FunctionCall {
                        identifier: format!("__builtin__{identifier}"),
                        nargs: *nargs,
                    })
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
