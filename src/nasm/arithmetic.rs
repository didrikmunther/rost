use crate::compiler::definition::Arithmetic;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    fn get_equality_operations(procedure: &str, arithmetic: &Arithmetic) -> Vec<Row> {
        let label = Self::get_procedure_name(procedure, Some("equality"));
        let jump = match arithmetic {
            Arithmetic::Equality => Row::JumpIfEquals(label.clone()),
            Arithmetic::LessThan => Row::JumpIfLessThan(label.clone()),
            Arithmetic::GreaterThan => Row::JumpIfGreaterThan(label.clone()),
            _ => unreachable!(),
        };

        vec![
            Row::Compare("rax".into(), "rbx".into()),
            Row::Move("rax".into(), "1".into()),
            jump,
            Row::Move("rax".into(), "0".into()),
            Row::Label(label),
        ]
    }

    pub fn handle_arithmetic(
        &mut self,
        procedure: &str,
        arithmetic: &Arithmetic,
    ) -> Result<(), NasmError> {
        let operations = match arithmetic {
            Arithmetic::Add => vec![Row::Add("rax".into(), "rbx".into())],
            Arithmetic::Subtract => vec![Row::Subtract("rax".into(), "rbx".into())],
            Arithmetic::Multiply => vec![Row::Multiply("rbx".into())],
            Arithmetic::Divide => vec![
                Row::Xor("rdx".into(), "rdx".into()),
                Row::Divide("rbx".into()),
            ],
            Arithmetic::Equality | Arithmetic::LessThan | Arithmetic::GreaterThan => {
                Self::get_equality_operations(procedure, arithmetic)
            }
            #[allow(unreachable_patterns)]
            _ => panic!("Unimplemented: {:?}", arithmetic),
        };

        self.code
            .add(Row::Pop("rax".into()))
            .add(Row::Pop("rbx".into()));

        for operation in operations {
            self.code.add(operation);
        }

        self.code.add(Row::Push("rax".into()));

        Ok(())
    }
}
