use crate::compiler::definition::Function;

use super::{code::Code, error::NasmError, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn save_base_pointer(&mut self) -> &mut Code {
        self.code
            .add(Row::Comment("Save base pointer".into()))
            .add(Row::Push("rbp".into()))
            .add(Row::Move("rbp".into(), "rsp".into()))
    }

    pub fn restore_base_pointer(&mut self) -> &mut Code {
        self.code
            .add(Row::Comment("Restore base pointer".into()))
            .add(Row::Move("rsp".into(), "rbp".into()))
            .add(Row::Pop("rbp".into()))
    }

    pub fn handle_function_declaration(
        &mut self,
        index: usize,
        function: &Function,
    ) -> Result<(), NasmError> {
        let name = Self::get_function_name(index);

        let old_stack_pos = self.code.stack_pos;

        self.code.add(Row::Label(name.clone()));

        self.save_base_pointer();

        self.code.stack_pos = 1;
        self.add_program(&function.body, &name)?;
        self.code.stack_pos = old_stack_pos;

        self.restore_base_pointer()
            .add(Row::Comment("Return from function".into()))
            .add(Row::Ret);

        Ok(())
    }
}
