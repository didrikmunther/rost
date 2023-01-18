use crate::lexer::{Keyword, Token};

use super::{
    definition::{Declaration, DeclarationKind, FunctionDeclaration, FunctionDeclarationParameter},
    error::{ParserError, ParserErrorKind},
    parser::Parser,
    util::get_block_identifier,
};

impl<'a> Parser<'a> {
    pub fn function_declaration(&mut self) -> Result<Declaration, ParserError> {
        if let Some(_) = self.get(&[Keyword::Fn]) {
            let fn_identifier = self.expect(&[Keyword::Identifier])?;
            let identifier = match get_block_identifier(&fn_identifier) {
                Some(identifier) => identifier,
                _ => {
                    return Err(ParserError::new(
                        fn_identifier.pos.clone(),
                        ParserErrorKind::Expected(&[Keyword::Identifier]),
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

                if let None = self.get(&[Keyword::Comma]) {
                    if let Some(close) = self.get(&[Keyword::ParRight]) {
                        let return_type = self
                            .get(&[Keyword::Arrow])
                            .map(|_| self.parse_type())
                            .transpose()?;

                        self.expect(&[Keyword::BracketLeft])?;
                        let mut content: Vec<Declaration> = Vec::new();

                        while let None = self.get(&[Keyword::BracketRight]) {
                            content.push(self.declaration()?);
                        }

                        return Ok(Declaration {
                            pos: fn_identifier.pos.start..close.pos.end,
                            kind: DeclarationKind::FunctionDeclaration(FunctionDeclaration {
                                identifier,
                                parameters,
                                content,
                                return_type,
                            }),
                        });
                    }
                }

                let (par_identifier, par_identifier_pos) =
                    if let Some(identifier) = self.get(&[Keyword::Identifier]) {
                        if let Token::Identifier(ref s) = identifier.token {
                            (s, &identifier.pos)
                        } else {
                            return Err(ParserError::new(
                                fn_identifier.pos.clone(),
                                ParserErrorKind::Expected(&[Keyword::Identifier]),
                            ));
                        }
                    } else {
                        return Err(ParserError::new(
                            fn_identifier.pos.clone(),
                            ParserErrorKind::Expected(&[Keyword::Identifier]),
                        ));
                    };

                self.expect(&[Keyword::Colon])?;
                let par_type = self.parse_type()?;

                parameters.push(FunctionDeclarationParameter {
                    identifier: par_identifier.clone(),
                    typ: par_type,
                    pos: par_identifier_pos.clone(),
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
