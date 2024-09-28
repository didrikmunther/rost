use super::{Letter, Lexer, LexerError, Token};

pub struct SingleLineCommentLexer;
pub struct MultiLineCommentLexer;

impl Lexer for SingleLineCommentLexer {
    fn lex(&self, chars: &[Letter]) -> Result<Option<(Token, usize)>, LexerError> {
        let mut buf = Vec::<char>::new();
        let mut is_comment = false;

        for (i, &(_pos, cur, eof)) in chars.iter().enumerate() {
            if !is_comment {
                if buf.is_empty() && cur.is_whitespace() {
                    continue;
                }

                if buf.len() >= 2 {
                    return Ok(None);
                }

                if buf.len() == 1 && cur == '/' {
                    is_comment = true;
                    buf.clear();
                    continue;
                }
            } else if eof || cur == '\n' {
                return Ok(Some((Token::Comment(buf.iter().collect()), i)));
            }

            buf.push(cur);
        }

        Ok(None)
    }
}

impl Lexer for MultiLineCommentLexer {
    fn lex(&self, chars: &[Letter]) -> Result<Option<(Token, usize)>, LexerError> {
        let mut buf = Vec::<char>::new();
        let mut is_comment = false;

        for (i, (&(_, prev, _), &(_, cur, eof))) in
            chars.iter().zip(chars.iter().skip(1)).enumerate()
        {
            if !is_comment {
                if prev.is_whitespace() {
                    continue;
                }

                if prev == '/' && cur == '*' {
                    is_comment = true;
                    continue;
                } else {
                    return Ok(None);
                }
            } else if eof || (prev == '*' && cur == '/') {
                return Ok(Some((
                    Token::Comment(buf.iter().take(buf.len() - 1).collect()),
                    i + 2,
                )));
            }

            buf.push(cur);
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::letter::get_letters;

    #[test]
    fn comment_works_eof() {
        let letters = &get_letters("//  hej");
        let lexed = SingleLineCommentLexer.lex(letters);

        assert_eq!(lexed, Ok(Some((Token::Comment(String::from("  hej")), 7))));
    }

    #[test]
    fn comment_works_newline() {
        let letters = &get_letters("//  hej\nnewline");
        let lexed = SingleLineCommentLexer.lex(letters);

        assert_eq!(lexed, Ok(Some((Token::Comment(String::from("  hej")), 7))));
    }

    #[test]
    fn multiline_comment_works() {
        let letters = &get_letters("  /* hej \n san */  \n abc");
        let lexed = MultiLineCommentLexer.lex(letters);

        assert_eq!(
            lexed,
            Ok(Some((Token::Comment(String::from(" hej \n san ")), 17)))
        );
    }
}
