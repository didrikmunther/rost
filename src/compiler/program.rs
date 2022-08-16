use super::asm::ASMGenererator;

struct GlobalData {
    pub content: String, // todo: can be all sorts of bytes
}

pub struct Program {
    global_data: Vec<GlobalData>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            global_data: Vec::new(),
        }
    }

    pub fn asm(&self) -> ASMGenererator {
        ASMGenererator::new(self)
    }
}