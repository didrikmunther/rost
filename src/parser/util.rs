use crate::lexer::{Block, Token};

use super::definition::{Expression, ExpressionKind, Primary};

pub fn get_expr_identifier(expr: &Expression) -> Option<String> {
    match expr.kind {
        ExpressionKind::Primary(ref primary) => match primary {
            Primary::Identifier(identifier) => Some(identifier.clone()),
            _ => None,
        },
        _ => None,
    }
}

pub fn get_block_identifier(block: &Block) -> Option<String> {
    match &block.token {
        Token::Identifier(identifier) => Some(identifier.clone()),
        _ => None,
    }
}
