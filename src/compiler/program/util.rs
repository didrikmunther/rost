use super::{
    builder::Builder,
    ir::{Variable, VariableId},
    scope::ScopedVariable,
    Program,
};
use crate::{compiler::error::CompilerError, parser::definition::Declaration};

impl Program {
    pub fn get_instructions(&mut self, content: &[Declaration]) -> Result<Builder, CompilerError> {
        let mut builder = Builder::default();

        for declaration in content {
            let result = self.handle_declaration(declaration);
            if let Some(result) = self.get_or_add_error(result) {
                builder = builder.append(result);
            }
        }

        Ok(builder)
    }

    fn _insert_variable<F>(
        &mut self,
        variable: Variable,
        identifier: Option<&str>,
        scope: F,
    ) -> VariableId
    where
        F: FnOnce(VariableId) -> ScopedVariable,
    {
        let variable_id: VariableId = self.variables.len();

        if let Some(identifier) = identifier {
            self.get_scope_mut()
                .variable_lookup
                .insert(identifier.to_string(), scope(variable_id));
        }

        self.variables.push(variable);

        variable_id
    }

    pub fn insert_variable(&mut self, variable: Variable, identifier: Option<&str>) -> VariableId {
        self._insert_variable(variable, identifier, ScopedVariable::Native)
    }

    pub fn insert_argument(&mut self, variable: Variable, identifier: Option<&str>) -> VariableId {
        self._insert_variable(variable, identifier, ScopedVariable::Argument)
    }

    pub fn get_variable(&self, identifier: &str) -> Option<ScopedVariable> {
        self.get_scope().variable_lookup.get(identifier).cloned()
    }
}
