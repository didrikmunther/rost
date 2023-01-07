use super::{
    builder::Builder,
    definition::{Function, GlobalData, Procedure, ProcedureCall, ProcedureKind},
    error::CompilerError,
    scope::FunctionScope,
};

use crate::parser::definition::{Declaration, DeclarationKind, FunctionDeclaration, ReturnType};

#[derive(Debug)]
pub struct Program {
    pub function_scope: FunctionScope,
    pub global_data: Vec<GlobalData>,
    pub functions: Vec<Function>,
    pub procedures: Builder,
    pub stack_pos: usize,
}

impl Program {
    pub fn new() -> Self {
        Self {
            function_scope: FunctionScope::new(ReturnType::None),
            global_data: Vec::new(),
            functions: Vec::new(),
            procedures: Builder::new(),
            stack_pos: 0,
        }
    }

    pub fn get_return_type(&self) -> &ReturnType {
        &self.function_scope.return_type
    }

    /// Used when entering a new function scope.
    /// Makes sure that variables are properly scoped.
    pub fn with_function_scope<F>(
        &mut self,
        return_type: ReturnType,
        inner: F,
    ) -> Result<Builder, CompilerError>
    where
        F: FnOnce(&mut Self) -> Result<Builder, CompilerError>,
    {
        let mut function_scope = FunctionScope::new(return_type);
        std::mem::swap(&mut function_scope, &mut self.function_scope);
        self.function_scope.set_parent(Box::new(function_scope));

        let result = inner(self)?;

        self.function_scope = self.function_scope.take_parent();

        Ok(result)
    }

    /// Used when entering a new scope.
    /// Makes sure that variables are properly scoped.
    pub fn with_scope<F>(&mut self, inner: F) -> Result<Builder, CompilerError>
    where
        F: FnOnce(&mut Self) -> Result<Builder, CompilerError>,
    {
        self.function_scope.create_scope();
        let result = inner(self)?;
        self.function_scope.close_scope();

        Ok(result)
    }

    pub fn compile(mut self, parsed: Vec<Declaration>) -> Result<Program, CompilerError> {
        let main_func = Declaration {
            pos: 0..0,
            kind: DeclarationKind::FunctionDeclaration(FunctionDeclaration {
                identifier: "main".into(),
                parameters: vec![], // todo: add argv, argc
                content: parsed,
                return_type: ReturnType::None,
            }),
        };

        self.handle_declaration(&main_func)?; // Add main function to program

        let main_func_id = self.functions.len() - 1; // Main function will always be last
        let call_main_func = Procedure::new(
            0..0,
            ProcedureKind::ProcedureCall(ProcedureCall {
                function_id: main_func_id,
                nargs: 0,
                returns: false,
            }),
        );

        self.procedures = Builder::new().push(call_main_func); // Call main function

        Ok(self)
    }
}
