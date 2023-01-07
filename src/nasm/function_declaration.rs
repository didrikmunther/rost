use crate::compiler::definition::Function;

use super::{error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn handle_function_declaration(
        &mut self,
        index: usize,
        function: &Function,
    ) -> Result<(), NasmError> {
        let name = Self::get_function_name(index);

        // let par_offset = 1 + function.npars; // Compensate for the return address on the stack from CALL instruction
        let old_stack_pos = self.code.stack_pos;
        let misaligned = self.code.stack_pos % 2 == 0;
        // self.code.stack_pos += par_offset; // Let the stack begin at the first argument of the function

        self.code.add(Row::Label(name.clone()));

        // if misaligned {
        //     self.code
        //         .add_with_comment(Row::Push("rbp".into()), "Dummy to align to 16".into());
        // }

        self.code
            .add(Row::Comment("Save base pointer".into()))
            .add(Row::Push("rbp".into()))
            .add(Row::Move("rbp".into(), "rsp".into()));

        self.code.stack_pos = 1;
        self.add_program(&function.body, &name)?;
        self.code.stack_pos = old_stack_pos;

        self.code
            .add(Row::Comment("Restore base pointer".into()))
            .add(Row::Move("rsp".into(), "rbp".into()))
            .add(Row::Pop("rbp".into()));

        // if misaligned {
        //     self.code
        //         .add_with_comment(Row::Pop("rbp".into()), "Dummy to remove alignment".into());
        // }

        self.code
            .add(Row::Comment("Return from function".into()))
            .add(Row::Ret);

        Ok(())
    }
}
