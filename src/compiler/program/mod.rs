use super::error::CompilerError;
use crate::{
    lexer::Keyword,
    parser::{
        definition::{Declaration, FunctionDeclaration, FunctionDeclarationParameter},
        types::{Type, TypeIdentifier},
    },
};
use builder::Builder;
use ir::{PrimitiveValue, Variable};
use rust_lapper::Lapper;
use scope::{Scope, ScopeId, ScopeRange};

mod assignment;
pub mod builder;
mod declaration;
mod expression;
mod function_call;
mod function_declaration;
pub mod ir;
mod scope;
mod util;
mod variable;

pub type Location = usize;

#[derive(Debug)]
pub struct Program {
    pub instructions: Builder,
    pub variables: Vec<Variable>,
    pub global_data: Vec<PrimitiveValue>,

    pub scopes: Vec<Scope>,
    pub scope_id: ScopeId,
    // This is quite a hack, but it's the easiest way to get the scope of a variable.
    // We should use some kind of run-length encoding to make this more efficient.
    scope_ranges: Vec<ScopeRange>,
    pub scope_lookup: Lapper<Location, ScopeId>,

    pub errors: Vec<CompilerError>,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::default()],
            scope_id: 0,
            scope_lookup: Lapper::new(vec![]),
            scope_ranges: vec![],
            instructions: Builder::new(),
            variables: vec![],
            global_data: vec![],
            errors: vec![],
        }
    }

    pub fn get_or_add_error(&mut self, result: Result<Builder, CompilerError>) -> Option<Builder> {
        match result {
            Ok(builder) => Some(builder),
            Err(error) => {
                self.errors.push(error);
                None
            }
        }
    }

    fn add_builtin_functions(&mut self) -> Result<(), CompilerError> {
        self.create_function_declaration(&FunctionDeclaration {
            identifier: "printf".to_string(),
            identifier_pos: 0..0,
            parameters: vec![FunctionDeclarationParameter {
                identifier: "format".to_string(),
                typ: Type {
                    identifier: TypeIdentifier::Primitive(Keyword::String),
                    pos: 0..0,
                    children: None,
                },
                pos: 0..0,
            }],
            content: vec![],
            return_type: None,
        })?;

        Ok(())
    }

    fn _compile(&mut self, parsed: Vec<Declaration>) -> Result<(), CompilerError> {
        let location = parsed
            .iter()
            .fold(0..0, |acc, decl| acc.start..decl.pos.end);

        self.scope_ranges.push((location, self.scope_id));
        self.add_builtin_functions()?;
        self.instructions = self.get_instructions(&parsed)?;

        Ok(())
    }

    pub fn compile(mut self, parsed: Vec<Declaration>) -> Program {
        let result = self._compile(parsed);
        self.scope_lookup = self.get_scope_lookup();

        if let Err(error) = result {
            self.errors.push(error);
        }

        self
    }
}
