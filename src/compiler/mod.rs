use self::program::Program;
use crate::parser::definition::Declaration;

pub mod error;
pub mod program;

pub fn compile(parsed: Vec<Declaration>) -> Program {
    Program::new().compile(parsed)
}
