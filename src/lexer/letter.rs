use super::LexerError;

use std::iter;

pub type Letter = (usize, char, bool);

pub static EOF: Letter = (0, ' ', true);

pub fn get_letters<'a>(text: &'a str) -> Vec<Letter> {
    text
        .chars()
        .enumerate()
        .map(|(i, v)| (i, v, false))
        .chain(iter::once(EOF))
        .collect::<Vec<Letter>>()
}

pub trait UnexpectedToken {
    fn unexpected_token(&self) -> LexerError;
}

impl UnexpectedToken for Letter {
    fn unexpected_token(&self) -> LexerError {
        LexerError {
            pos: self.0..self.0+1,
            message: format!("Unexpected token: {:?}", self.1)
        }
    }
}