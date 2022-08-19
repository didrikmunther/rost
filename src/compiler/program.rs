use std::{collections::HashMap, ops::Range};

use crate::{lexer::Keyword, parser::definition::Declaration};

use super::{builder::Builder, definition::GlobalData, error::CompilerError};

#[derive(Debug)]
pub struct Variable {
    pub pos: Range<usize>,
    pub typ: Keyword,
    pub stack_pos: usize,
}

#[derive(Debug)]
pub struct Program {
    pub global_data: Vec<GlobalData>,
    pub procedures: Builder,

    pub stack_pos: usize,
    pub variables: HashMap<String, Variable>, // stack position
}

impl Program {
    pub fn new() -> Self {
        Self {
            global_data: Vec::new(),
            procedures: Builder::new(),

            stack_pos: 0,
            variables: HashMap::new(),
        }
    }

    pub fn compile(mut self, parsed: &Vec<Declaration>) -> Result<Program, CompilerError> {
        for declaration in parsed {
            let procedure = self.handle_declaration(declaration)?;
            self.procedures = self.procedures.append(procedure);
        }

        Ok(self)
    }
}
