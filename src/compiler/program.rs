use std::ops::Range;

use crate::{
    lexer::Literal,
    parser::{
        Declaration, DeclarationKind, Expression, ExpressionKind, FunctionCall, Primary,
        StatementKind,
    },
};

use super::error::CompilerError;

#[derive(Debug)]
pub struct Procedure {
    pub pos: Range<usize>,
    pub kind: ProcedureKind,
}

#[derive(Debug)]
pub enum ProcedureKind {
    SystemCall(SystemCall),
}

#[derive(Debug)]
pub struct SystemCall {
    pub identifier: String,
    pub args: Vec<usize>, // todo: this is just id for data labels, args can come from registers as well.
}

#[derive(Debug)]
pub struct GlobalData {
    pub content: String, // todo: can be all sorts of bytes
}

#[derive(Debug)]
pub struct Program {
    pub global_data: Vec<GlobalData>,
    pub procedures: Vec<Procedure>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            global_data: Vec::new(),
            procedures: Vec::new(),
        }
    }

    fn handle_fcall(&mut self, fcall: &FunctionCall) -> Result<SystemCall, CompilerError> {
        let mut args: Vec<usize> = Vec::new();

        for arg in &fcall.args {
            match &arg.kind {
                ExpressionKind::Primary(primary) => match primary {
                    Primary::Literal(literal) => match literal {
                        Literal::String(s) => {
                            self.global_data.push(GlobalData { content: s.clone() });
                            args.push(self.global_data.len() - 1); // push latest index
                        }
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }

        let identifier = match &fcall.identifier.kind {
            ExpressionKind::Primary(primary) => match primary {
                Primary::Identifier(s) => s.clone(),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        };

        Ok(SystemCall { identifier, args })
    }

    fn handle_expression(&mut self, expression: &Expression) -> Result<Procedure, CompilerError> {
        match &expression.kind {
            ExpressionKind::FunctionCall(fcall) => Ok(Procedure {
                pos: expression.pos.clone(),
                kind: ProcedureKind::SystemCall(self.handle_fcall(fcall)?),
            }),
            _ => unimplemented!(),
        }
    }

    pub fn handle_declaration(
        &mut self,
        declaration: &Declaration,
    ) -> Result<Procedure, CompilerError> {
        match &declaration.kind {
            DeclarationKind::Statement(statement) => match &statement.kind {
                StatementKind::Expression(expression) => self.handle_expression(expression),
            },
        }
    }

    pub fn compile(mut self, parsed: &Vec<Declaration>) -> Result<Program, CompilerError> {
        for declaration in parsed {
            let procedure = self.handle_declaration(declaration)?;
            self.procedures.push(procedure);
        }

        Ok(self)
    }
}
