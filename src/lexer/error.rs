use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct LexerError {
    pub pos: Range<usize>,
    pub message: String,
}

impl LexerError {
    pub fn get_error(&self, text: &str) -> String {
        let text = String::from(text);

        let newlines = text
            .chars()
            .enumerate()
            .take(self.pos.start)
            .filter(|&(_, v)| v == '\n')
            .map(|(i, _)| i);

        let line = newlines.clone().count();
        let line_pos = newlines
            .clone()
            .last()
            .and_then(|i| Some(self.pos.start - i))
            .unwrap_or(0);

        let margin: usize = 1;

        let caret = format!(
            "{}{}",
            String::from(" ").repeat(line_pos - 1),
            String::from("^").repeat(self.pos.end - self.pos.start),
        );

        let msg = format!(
            "{}└─ {}",
            String::from(" ").repeat(line_pos - 1),
            self.message
        );

        let lines: String = text
            .lines()
            .enumerate()
            .skip(std::cmp::max(line as i32 - margin as i32, 0) as usize)
            .take(1 + margin * 2)
            .fold(String::new(), |acc, (i, v)| {
                if i == line {
                    format!("{}{} | {}\n  | {}\n  | {}\n", acc, i + 1, v, caret, msg)
                } else {
                    format!("{}{} | {}\n", acc, i + 1, v)
                }
            });

        let header = format!("  --> [main]:{}:{}", line + 1, line_pos + 1);

        format!("{}\n{}", header, lines)
    }
}
