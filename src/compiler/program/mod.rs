use super::error::CompilerError;
use crate::parser::definition::Declaration;
use builder::Builder;
use ir::{Function, FunctionTemplate, PrimitiveValue, Type, TypeId, TypeKind, Variable};
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
    pub functions: Vec<Function>,
    pub function_templates: Vec<FunctionTemplate>,
    pub types: Vec<Type>,
    pub global_data: Vec<PrimitiveValue>,

    pub scopes: Vec<Scope>,
    pub scope_id: ScopeId,
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
            functions: vec![],
            function_templates: vec![],
            types: vec![],
            global_data: vec![],
            errors: vec![],
        }
    }

    pub fn add_error(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    pub fn get_or_add_error<T>(&mut self, result: Result<T, CompilerError>) -> Option<T> {
        match result {
            Ok(v) => Some(v),
            Err(error) => {
                self.errors.push(error);
                None
            }
        }
    }

    fn add_intrinsic_type(&mut self, identifier: &str) -> TypeId {
        self.insert_type(
            Some(identifier),
            Type {
                kind: TypeKind::Intrinsic,
                type_parameters: vec![],
            },
        )
    }

    fn add_builtin_types(&mut self) {
        self.add_intrinsic_type("int");
        self.add_intrinsic_type("float");
        self.add_intrinsic_type("str");
        self.add_intrinsic_type("bool");
        self.add_intrinsic_type("any");
    }

    fn _compile(&mut self, parsed: Vec<Declaration>) -> Result<(), CompilerError> {
        let location = parsed
            .iter()
            .fold(0..0, |acc, decl| acc.start..decl.pos.end);

        self.scope_ranges.push((location, self.scope_id));
        self.add_builtin_types();
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
