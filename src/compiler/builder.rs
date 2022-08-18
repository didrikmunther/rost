use std::collections::LinkedList;

use super::definition::Procedure;

#[derive(Debug)]
pub struct Builder {
    pub procedures: LinkedList<Procedure>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            procedures: LinkedList::new(),
        }
    }

    pub fn push(mut self, instruction: Procedure) -> Self {
        self.procedures.push_back(instruction);
        self
    }

    pub fn append(mut self, mut builder: Builder) -> Self {
        self.procedures.append(&mut builder.procedures);
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = &Procedure> {
        self.procedures.iter()
    }
}
