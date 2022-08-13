use super::{Letter, Lexer, LexerError, Token};

pub struct CommentLexer;

impl CommentLexer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lexer for CommentLexer {
    fn lex<'a>(&self, chars: &'a [Letter]) -> Result<Option<(Token, &'a [Letter])>, LexerError> {
        let mut buf = Vec::<char>::new();
        let mut is_comment = false;

        for (i, &(_pos, cur, eof)) in chars.into_iter().enumerate() {
            if buf.is_empty() && cur.is_whitespace() {
                continue;
            }

            if !is_comment && buf.len() >= 2 {
                return Ok(None);
            }

            if !is_comment && buf.len() == 1 && cur == '/' {
                is_comment = true;
                buf.clear();
                continue;
            }

            if is_comment && (eof || cur == '\n') {
                return Ok(Some((Token::Comment(buf.iter().collect()), &chars[i..])));
            }

            buf.push(cur);
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::letter::{get_letters, EOF};

    #[test]
    fn comment_works() {
        let letters = &get_letters("// hej");
        let lexed = CommentLexer::new().lex(letters);
        let rest: &[Letter] = &[EOF];

        assert_eq!(lexed, Ok(Some((Token::Comment(String::from("hej")), rest))));
    }
}
