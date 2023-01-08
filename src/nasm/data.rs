use crate::compiler::definition::GlobalData;

use super::{code::Code, generator::Generator, row::Row};

impl<'a> Generator<'a> {
    fn add_data_section(&mut self) {
        self.code.add(Row::Section("data".into()));

        for (label, data) in self.program.global_data.iter() {
            match data {
                GlobalData::String(s) => {
                    self.code.add(Row::Label(label.clone()));
                    self.code.add(Row::DeclareStaticString(s.clone()));
                }
                GlobalData::Reserved(_) => {}
            };
        }
    }

    fn add_bss_section(&mut self) {
        self.code.add(Row::Section("bss".into()));

        for (label, data) in self.program.global_data.iter() {
            match data {
                GlobalData::Reserved(i) => {
                    self.code.add(Row::Label(label.clone()));
                    self.code.add(Row::ReserveBytes(i * 8));
                }
                GlobalData::String(_) => {}
            };
        }
    }

    pub fn add_data(&mut self) -> &mut Code {
        self.add_bss_section();
        self.add_data_section();

        &mut self.code
    }
}
