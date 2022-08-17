use crate::parser::definition::Declaration;

use super::{
    definition::{GlobalData, Procedure},
    error::CompilerError,
};

#[derive(Debug)]
pub struct Program {
    pub global_data: Vec<GlobalData>,
    pub procedures: Vec<Procedure>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            global_data: Vec::new(),
            procedures: Vec::new(),
        }
    }

    pub fn compile(mut self, parsed: &Vec<Declaration>) -> Result<Program, CompilerError> {
        for declaration in parsed {
            let procedure = self.handle_declaration(declaration)?;
            self.procedures.push(procedure);
        }

        Ok(self)
    }
}
