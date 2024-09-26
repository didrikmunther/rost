use std::collections::LinkedList;

use super::ir::Instruction;

#[derive(Debug, Default)]
pub struct Builder {
    pub instructions: LinkedList<Instruction>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, instruction: Instruction) -> Self {
        self.instructions.push_back(instruction);
        self
    }

    pub fn append(mut self, mut builder: Builder) -> Self {
        self.instructions.append(&mut builder.instructions);
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = &Instruction> {
        self.instructions.iter()
    }
}
