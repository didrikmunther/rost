use super::{
    builder::Builder,
    definition::{Function, GlobalData, Procedure, ProcedureCall, ProcedureKind},
    error::{CompilerError, CompilerErrorKind},
    scope::{
        function_scope::FunctionScope, root_scope::RootScope, variable::VariableType, ProgramScope,
    },
};

use crate::parser::definition::{Declaration, ReturnType};

#[derive(Debug)]
pub struct Program {
    pub scope: ProgramScope,
    pub global_data: Vec<GlobalData>,
    pub functions: Vec<Function>,
    pub procedures: Builder,
    pub stack_pos: usize,
}

impl Program {
    pub fn new() -> Self {
        Self {
            scope: ProgramScope::RootScope(RootScope::new()),
            global_data: Vec::new(),
            functions: Vec::new(),
            procedures: Builder::new(),
            stack_pos: 0,
        }
    }

    /// Used when entering a new function scope.
    /// Makes sure that variables are properly scoped.
    // todo: clean up
    pub fn with_function_scope<F>(
        &mut self,
        return_type: ReturnType,
        inner: F,
    ) -> Result<Builder, CompilerError>
    where
        F: FnOnce(&mut Self) -> Result<Builder, CompilerError>,
    {
        let mut function_scope = ProgramScope::FunctionScope(FunctionScope::new(return_type));
        std::mem::swap(&mut function_scope, &mut self.scope);

        let ProgramScope::FunctionScope(own_function_scope) = &mut self.scope else {
            unreachable!();
        };

        own_function_scope.set_parent(Box::new(function_scope));

        let result = inner(self)?;

        let ProgramScope::FunctionScope(own_function_scope) = &mut self.scope else {
            unreachable!();
        };

        self.scope = own_function_scope.take_parent();

        Ok(result)
    }

    /// Used when entering a new scope.
    /// Makes sure that variables are properly scoped.
    pub fn with_scope<F>(&mut self, inner: F) -> Result<Builder, CompilerError>
    where
        F: FnOnce(&mut Self) -> Result<Builder, CompilerError>,
    {
        // Todo: Add struct scope, in which you define fields and functions for structs.
        match &mut self.scope {
            ProgramScope::RootScope(scope) => {
                scope.create_scope();
            }
            ProgramScope::FunctionScope(scope) => {
                scope.create_scope();
            }
        }

        let result = inner(self)?;

        match &mut self.scope {
            ProgramScope::RootScope(scope) => {
                scope.close_scope();
            }
            ProgramScope::FunctionScope(scope) => {
                scope.close_scope();
            }
        }

        Ok(result)
    }

    pub fn compile(mut self, parsed: Vec<Declaration>) -> Result<Program, CompilerError> {
        let procedures = self.get_procedures(&parsed)?;

        #[allow(irrefutable_let_patterns)] // todo: remove
        let ProgramScope::RootScope(root_scope) = &self.scope else {
            todo!("We should be in root_scope by end of program");
        };

        let Some(&VariableType::Function(main_func_id)) = root_scope.scope.variables.get("main").map(|var| &var.typ) else {
            return Err(CompilerError::new(0..0, CompilerErrorKind::MissingMainFunction))
        };

        let call_main_func = Procedure::new(
            0..0,
            ProcedureKind::ProcedureCall(ProcedureCall {
                function_id: main_func_id,
                nargs: 0,
                returns: false,
            }),
        );

        let builder = Builder::new()
            .push(Procedure::new(
                0..0,
                ProcedureKind::Allocate(root_scope.variables.len()),
            ))
            .append(procedures)
            .push(call_main_func);

        self.procedures = builder;

        Ok(self)
    }
}
