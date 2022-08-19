use super::{code::Code, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn add_data(&mut self) -> &mut Code {
        self.code.add(Row::Section("data".into()));

        for (i, data) in self.program.global_data.iter().enumerate() {
            self.code
                .add(Row::Label(Self::get_data_name(i)))
                .add(Row::DeclareStaticString(data.content.clone()));
        }

        &mut self.code
    }
}
