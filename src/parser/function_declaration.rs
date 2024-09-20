use super::{
    definition::{
        Declaration, DeclarationKind, FunctionDeclaration, FunctionDeclarationContent,
        FunctionDeclarationParameter,
    },
    error::{ParserError, ParserErrorKind},
    util::get_block_identifier,
    Parser,
};
use crate::lexer::Keyword;

impl<'a> Parser<'a> {
    pub fn function_declaration(&mut self) -> Result<Declaration, ParserError> {
        let builtin = self.get(&[Keyword::Builtin]).is_some();
        if builtin {
            self.expect(&[Keyword::Fn])?;
        }

        if builtin || self.get(&[Keyword::Fn]).is_some() {
            let mut vararg_parameter: Option<FunctionDeclarationParameter> = None;
            let fn_identifier = self.expect(&[Keyword::Identifier])?;

            let identifier = match get_block_identifier(fn_identifier) {
                Some(identifier) => identifier,
                _ => {
                    return Err(ParserError::new(
                        fn_identifier.pos.clone(),
                        ParserErrorKind::Expected {
                            expected: &[Keyword::Identifier],
                            got: fn_identifier.kind,
                        },
                    ))
                }
            };

            let par_open = self.expect(&[Keyword::ParLeft])?;
            let mut parameters = Vec::new();

            loop {
                if self.is_end() {
                    return Err(ParserError::new(
                        par_open.pos.clone(),
                        ParserErrorKind::UnterminatedPair(Keyword::ParLeft),
                    ));
                }

                if self.get(&[Keyword::Comma]).is_none() {
                    if let Some(close) = self.get(&[Keyword::ParRight]) {
                        let return_type = self
                            .get(&[Keyword::Arrow])
                            .map(|_| self.parse_type())
                            .transpose()?;

                        let content = if builtin {
                            self.expect(&[Keyword::Semicolon])?;

                            FunctionDeclarationContent::Builtin
                        } else {
                            self.expect(&[Keyword::BraceLeft])?;
                            let mut content: Vec<Declaration> = Vec::new();

                            while self.get(&[Keyword::BraceRight]).is_none() {
                                content.push(self.declaration()?);
                            }

                            FunctionDeclarationContent::Block(content)
                        };

                        return Ok(Declaration {
                            pos: fn_identifier.pos.start..close.pos.end,
                            kind: DeclarationKind::FunctionDeclaration(FunctionDeclaration {
                                identifier,
                                identifier_pos: fn_identifier.pos.clone(),
                                parameters,
                                vararg_parameter,
                                content,
                                return_type,
                            }),
                        });
                    }
                }

                if vararg_parameter.is_some() {
                    let next = self.peek().unwrap();

                    return Err(ParserError::new(
                        next.pos.clone(),
                        ParserErrorKind::ParametersNotAllowedAfterVararg,
                    ));
                }

                if self.get(&[Keyword::Ellipsis]).is_some() {
                    let par_block = self.expect(&[Keyword::Identifier])?;
                    let par_identifier = self.extract_identifier(par_block)?;
                    self.expect(&[Keyword::Colon])?;
                    let par_type = self.parse_type()?;

                    vararg_parameter = Some(FunctionDeclarationParameter {
                        identifier: par_identifier.to_string(),
                        typ: par_type,
                        pos: par_block.pos.clone(),
                    });

                    continue;
                }

                let par_block = self.expect(&[Keyword::Identifier])?;
                let par_identifier = self.extract_identifier(par_block)?;
                self.expect(&[Keyword::Colon])?;
                let par_type = self.parse_type()?;

                parameters.push(FunctionDeclarationParameter {
                    identifier: par_identifier.to_string(),
                    typ: par_type,
                    pos: par_block.pos.clone(),
                });
            }
        }

        let statement = self.statement()?;

        Ok(Declaration {
            pos: statement.pos.clone(),
            kind: DeclarationKind::Statement(statement),
        })
    }
}
