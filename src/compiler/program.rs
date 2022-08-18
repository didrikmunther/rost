use std::{collections::HashMap};

use crate::parser::definition::Declaration;

use super::{
    definition::{GlobalData, Procedure},
    error::CompilerError,
};

#[derive(Debug)]
pub struct Program {
    pub global_data: Vec<GlobalData>,
    pub procedures: Vec<Procedure>,

    pub stack_pos: usize,
    pub variables: HashMap<String, usize>, // stack position
}

impl Program {
    pub fn new() -> Self {
        Self {
            global_data: Vec::new(),
            procedures: Vec::new(),

            stack_pos: 0,
            variables: HashMap::new(),
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
