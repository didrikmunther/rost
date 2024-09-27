use std::collections::HashMap;

use crate::compiler::program::{
    builder::Builder,
    ir::{FunctionBody, InstructionKind, PrimitiveValue, ProcedureCall, ValueKind, VariableId},
    Program,
};

use super::{
    code::{Code, Element, WasmType, WasmValue},
    error::WasmError,
};

pub enum FunctionType {
    Builtin,
    User,
}

pub struct Generator {
    function_types: HashMap<VariableId, FunctionType>,
    global_data_indexes: HashMap<usize, i32>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            function_types: HashMap::new(),
            global_data_indexes: HashMap::new(),
        }
    }

    pub fn generate_code(&mut self, program: &Program) -> Result<String, WasmError> {
        let Some(main_function) = program.get_scope().variable_lookup.get("main") else {
            todo!("Main function not found");
        };

        let root_element = Element::Block(vec![
            Element::Raw(include_str!("boilerplate_entry.wat").into()),
            self.get_setup(program)?,
            Element::Block(self.get_functions(program)?),
            Element::FunctionDefinition {
                identifier: "main".into(),
                export: false,
                content: Box::new(Element::Block(vec![
                    self.get_program(&program.instructions, program)?,
                    Element::FunctionCall(self.get_user_function_name(main_function.get_id())),
                ])),
            },
            Element::FunctionDefinition {
                identifier: "__main".into(),
                export: true,
                content: Box::new(Element::FunctionCall("main".into())),
            },
            Element::Raw(include_str!("boilerplate_exit.wat").into()),
        ]);

        let code = Code::new(root_element);

        Ok(format!("{}", code))
    }

    fn get_user_function_name(&self, variable_id: VariableId) -> String {
        format!("__userf__{}", variable_id)
    }

    pub fn get_functions(&mut self, program: &Program) -> Result<Vec<Element>, WasmError> {
        let mut elements = vec![Element::Comment("Function definitions begin".into())];

        for function in &program.functions {
            let var_identifier = &program.variables[function.variable_id].identifier;

            match &function.body {
                FunctionBody::Builtin(..) => {
                    self.function_types
                        .insert(function.variable_id, FunctionType::Builtin);
                }
                FunctionBody::Block {
                    body_contains_error: _body_contains_error,
                    content,
                } => {
                    elements.push(Element::Comment(format!(
                        "User function: {}",
                        var_identifier
                    )));

                    // Parameters are newly declared in the function,
                    // but we only care about non-parameter variables.
                    let declared_variables = function
                        .declared_variables
                        .iter()
                        .filter(|variable| !function.parameter_variable_ids.contains(variable));

                    let content = function
                        .parameter_variable_ids
                        .iter()
                        .map(|variable_id| {
                            let variable_name = program.variables[*variable_id].identifier.clone();
                            let typ = "i32";
                            Element::Raw(format!("(param $_{variable_id}_{variable_name} {typ})"))
                        })
                        .rev() // We need to reverse the order of the parameters in WASM
                        .chain(declared_variables.map(|variable_id| {
                            let variable_name = program.variables[*variable_id].identifier.clone();
                            let typ = "i32";
                            Element::Raw(format!("(local $_{variable_id}_{variable_name} {typ})"))
                        }))
                        .chain(std::iter::once(self.get_program(content, program)?))
                        .collect::<Vec<_>>();

                    elements.push(Element::FunctionDefinition {
                        identifier: format!("__userf__{}", function.variable_id),
                        export: false,
                        content: Box::new(Element::Block(content)),
                    });

                    self.function_types
                        .insert(function.variable_id, FunctionType::User);
                }
            }
        }

        Ok(elements)
    }

    pub fn get_setup(&mut self, program: &Program) -> Result<Element, WasmError> {
        let mut elements: Vec<Element> = vec![Element::Comment("Global data section begin".into())];

        let mut current_offset: i32 = 0;
        for (i, global_data) in program.global_data.iter().enumerate() {
            let (value, offset_delta) = match global_data {
                PrimitiveValue::String(value) => (format!("\"{}\\00\"", value), value.len() + 1),
                PrimitiveValue::Int(value) => (value.to_string(), 1),
                _ => todo!(),
            };

            elements.push(Element::Raw(format!(
                "(data (i32.const {current_offset}) {value})"
            )));

            self.global_data_indexes.insert(i, current_offset);

            current_offset += offset_delta as i32;
        }

        Ok(Element::Block(elements))
    }

    pub fn get_program(
        &self,
        instructions: &Builder,
        program: &Program,
    ) -> Result<Element, WasmError> {
        let mut elements: Vec<Element> = vec![];

        for instruction in instructions.iter() {
            match &instruction.kind {
                InstructionKind::Push(value) => match value {
                    ValueKind::Primitive(primitive) => match primitive {
                        PrimitiveValue::Int(value) => {
                            elements.push(Element::Push(WasmValue::I32(*value)));
                        }
                        // PrimitiveValue::String(value) => {
                        //     elements.push(Element::Push(format!("\"{}\"", value)))
                        // }
                        _ => todo!(),
                    },
                    ValueKind::GlobalData(id) => {
                        elements.push(Element::Push(WasmValue::I32(self.global_data_indexes[id])));
                    }
                    &ValueKind::Variable(id) => {
                        let variable_name = &program.variables[id].identifier;
                        let variable_name = format!("_{id}_{variable_name}");

                        // let value = match variable.scope {
                        //     VariableScope::Local => variable_name,
                        //     // VariableScope::Global => format!("__global_variables[{variable_name}]"),
                        // };

                        elements.push(Element::Push(WasmValue::Variable(variable_name)));
                    }
                },
                InstructionKind::Pop => elements.push(Element::Pop),
                InstructionKind::IntAdd => elements.push(Element::Add(WasmType::I32)),
                InstructionKind::IntMul => elements.push(Element::Mul(WasmType::I32)),
                &InstructionKind::Assign(id) => {
                    let variable_name = program.variables[id].identifier.clone();
                    elements.push(Element::Assign(format!("_{id}_{variable_name}")));
                }
                InstructionKind::ProcedureCall(ProcedureCall { variable_id, .. }) => {
                    if let Some(&FunctionType::Builtin) = self.function_types.get(variable_id) {
                        let identifier = program.variables[*variable_id].identifier.clone();
                        elements.push(Element::Comment(format!("Builtin call: {}", identifier)));
                        elements.push(Element::FunctionCall(identifier));
                    } else {
                        // todo!()
                        // elements.push(Element::Push(nargs.to_string()));
                        let identifier = program.variables[*variable_id].identifier.clone();
                        elements.push(Element::Comment(format!("Procedure call: {}", identifier)));
                        elements.push(Element::FunctionCall(
                            self.get_user_function_name(*variable_id),
                        ));
                    }
                }
                _ => {
                    todo!("Instruction: {:?}", instruction);
                }
            }
        }

        Ok(Element::Block(elements))
    }
}
