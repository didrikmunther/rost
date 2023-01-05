use crate::{
    lexer::Keyword,
    parser::definition::{Declaration, FunctionDeclaration},
};

use super::{builder::Builder, error::CompilerError, program::Program, scope::Variable};

#[derive(Debug)]
pub struct Function {
    pub npars: usize,
    pub body: Builder,
}

impl Program {
    pub fn handle_function_declaration(
        &mut self,
        statement: &Declaration,
        fdec: &FunctionDeclaration,
    ) -> Result<Builder, CompilerError> {
        self.new_scope();

        let npars = fdec.parameters.len();
        let old_stack_pos = self.stack_pos;

        for parameter in fdec.parameters.iter() {
            self.insert_variable(
                parameter.identifier.clone(),
                Variable {
                    pos: parameter.pos.clone(),
                    typ: parameter.typ,
                    stack_pos: self.stack_pos,
                },
            );

            self.stack_pos += 1;
        }

        self.stack_pos += 1; // Calling a function adds the RET address to the stack, temporarily compensate for this here.
        let body = self.get_procedures(&fdec.content)?;
        self.close_scope();
        self.stack_pos -= 1;

        self.functions.push(Function { body, npars });

        self.insert_variable(
            fdec.identifier.clone(),
            Variable {
                pos: statement.pos.clone(),
                typ: Keyword::Function,
                stack_pos: self.functions.len() - 1, // Store function id in stack_pos
            },
        );

        self.stack_pos = old_stack_pos;

        Ok(Builder::new())
    }
}
