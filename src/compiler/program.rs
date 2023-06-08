use std::collections::HashMap;

use super::{
    builder::Builder,
    definition::{Function, GlobalData, Procedure, ProcedureCall, ProcedureKind, Struct},
    error::{CompilerError, CompilerErrorKind},
    scope::{
        function_scope::FunctionScope,
        root_scope::RootScope,
        variable::{VariableLocation, VariableType},
        ProgramScope,
    },
};

use crate::{lexer::Keyword, parser::definition::Declaration};

#[derive(Debug)]
pub struct Program {
    pub scope: ProgramScope,
    pub global_data: HashMap<String, GlobalData>,
    pub functions: Vec<Function>,
    pub structs: Vec<Struct>,
    pub procedures: Builder,
    pub stack_pos: usize,

    // How many parameters does the main function take? Between 0 and 2
    pub main_func_nparams: usize,

    // Keep track of how many literal
    // values there are in global data section.
    pub literal_index: usize,
}

impl Program {
    pub fn new() -> Self {
        Self {
            scope: ProgramScope::RootScope(RootScope::new()),
            global_data: HashMap::new(),
            functions: Vec::new(),
            structs: Vec::new(),
            procedures: Builder::new(),
            stack_pos: 0,
            literal_index: 0,
            main_func_nparams: 0,
        }
    }

    /// Used when entering a new function scope.
    /// Makes sure that variables are properly scoped.
    // todo: clean up
    pub fn with_function_scope<F>(
        &mut self,
        return_type: Option<VariableType>,
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
        // Compile main program
        let procedures = self.get_procedures(&parsed)?;

        // Get root scope in order to find main function
        let ProgramScope::RootScope(root_scope) = &self.scope else {
            unreachable!("We should be in root_scope by end of program");
        };

        // Find main function in root scope
        // Todo: maybe main function shouldn't be required?
        let Some(&VariableType::Function(main_func_id)) = root_scope.scope.variables.get("main").map(|var| &var.typ) else {
            return Err(CompilerError::new(parsed.last().map(|v| v.pos.clone()).unwrap_or(0..0), CompilerErrorKind::MissingMainFunction))
        };

        let main_func = self
            .functions
            .get(main_func_id)
            .expect("Main function should exist in functions vec");

        // main_func_nparams represents the amount of parameters which are
        // requested by the main function (where parameters are argc, argv)
        // 0: []
        // 1: [int]
        // 2: [int, &&char]
        let first_type = VariableType::Value(Keyword::Int);
        let second_type = VariableType::Pointer(Box::new(VariableType::Pointer(Box::new(
            VariableType::Value(Keyword::Char),
        ))));

        // Parameters for the main function as slices of the type and the position of the type.
        let params = &main_func
            .parameters
            .iter()
            .map(|v| (self.get_variable_type(&v.typ), v.pos.clone()))
            .collect::<Vec<_>>()[..];

        self.main_func_nparams = match params {
            [] => 0,
            [_, _, (_, pos), ..] => {
                return Err(CompilerError::new(
                    pos.clone(),
                    CompilerErrorKind::TooManyParametersInMainFunction,
                ))
            }
            [(first, pos), ..] if *first != first_type => {
                return Err(CompilerError::new(
                    pos.clone(),
                    CompilerErrorKind::WrongType {
                        got: first.clone(),
                        expected: first_type,
                    },
                ))
            }
            [_] => 1,
            [_, (second, pos)] if *second != second_type => {
                return Err(CompilerError::new(
                    pos.clone(),
                    CompilerErrorKind::WrongType {
                        got: second.clone(),
                        expected: second_type,
                    },
                ))
            }
            [_, _] => 2,
        };

        // Create the call to the main function
        let call_main_func = Procedure::new(
            main_func.identifier_pos.clone(),
            ProcedureKind::ProcedureCall(ProcedureCall {
                function_id: main_func_id,
                nargs: self.main_func_nparams,
                returns: false,
            }),
        );

        // Add global variables to bss section
        for variable in root_scope.variables.values() {
            if let VariableType::Value(_) = variable.typ {
                #[allow(clippy::single_match)]
                match &variable.location {
                    VariableLocation::Global(label) => {
                        // Todo: currently only integers
                        self.global_data
                            .insert(label.clone(), GlobalData::Reserved(1));
                    }
                    _ => {}
                };
            }
        }

        let builder = Builder::new().append(procedures).push(call_main_func);

        self.procedures = builder;

        Ok(self)
    }
}
