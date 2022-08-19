use crate::lexer::{Block, Keyword, Literal, Token};

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

pub fn infer_type(expr: &Expression) -> Option<Keyword> {
    match &expr.kind {
        ExpressionKind::Primary(primary) => match primary {
            Primary::Literal(literal) => match literal {
                Literal::Int(_) => Some(Keyword::Int),
                Literal::Bool(_) => Some(Keyword::Bool),
                Literal::String(_) => Some(Keyword::String),
            },
            _ => None,
        },
        ExpressionKind::Binary(binary) => {
            let left = infer_type(&binary.left);
            let right = infer_type(&binary.right);

            if left == right {
                return left;
            }

            None
        }
        _ => None,
    }
}
