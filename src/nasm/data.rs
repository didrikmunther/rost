use crate::compiler::definition::GlobalData;

use super::{code::Code, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    pub fn add_data(&mut self) -> &mut Code {
        self.code.add(Row::Section("data".into()));

        for (label, data) in self.program.global_data.iter() {
            self.code.add(Row::Label(label.clone()));

            match data {
                GlobalData::String(s) => self.code.add(Row::DeclareStaticString(s.clone())),
                GlobalData::Int(i) => self.code.add(Row::DeclareStaticInt(*i)),
            };
        }

        &mut self.code
    }
}
