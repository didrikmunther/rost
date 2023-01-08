use self::{function_scope::FunctionScope, root_scope::RootScope};

pub mod function_scope;
pub mod root_scope;
pub mod scope;
pub mod variable;

#[derive(Debug)]
pub enum ProgramScope {
    FunctionScope(FunctionScope),
    RootScope(RootScope),
}
