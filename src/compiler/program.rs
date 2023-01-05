use std::collections::HashMap;

use super::{
    builder::Builder, definition::GlobalData, error::CompilerError, function_declaration::Function,
    scope::Scope,
};

use crate::parser::definition::{Declaration};

#[derive(Debug)]
pub struct Program {
    pub scope: Scope,
    pub global_data: Vec<GlobalData>,
    pub functions: Vec<Function>,
    pub procedures: Builder,
    pub stack_pos: usize,
}

impl Program {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
            global_data: Vec::new(),
            functions: Vec::new(),
            procedures: Builder::new(),
            stack_pos: 0,
        }
    }

    pub fn new_scope(&mut self) {
        let mut scope = Scope {
            parent: None,
            variables: HashMap::new(),
        };

        std::mem::swap(&mut scope, &mut self.scope);
        self.scope.parent = Some(Box::new(scope));
    }

    pub fn close_scope(&mut self) {
        self.scope = self.scope.take_parent();
    }

    pub fn compile(mut self, parsed: &Vec<Declaration>) -> Result<Program, CompilerError> {
        let mut procedures = Builder::new();

        for declaration in parsed {
            let procedure = self.handle_declaration(declaration)?;
            procedures = procedures.append(procedure);
        }

        self.procedures = procedures;

        Ok(self)
    }
}
