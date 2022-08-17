use std::{
    fmt::{Display, Formatter},
    ops::Range,
};

pub struct RostError {
    pos: Range<usize>,
    message: String,
    kind: String,
    file: Option<String>,
    code: Option<String>,
}

impl RostError {
    pub fn new(kind: String, message: String, pos: Range<usize>) -> Self {
        Self {
            pos,
            message,
            kind,
            file: None,
            code: None,
        }
    }

    pub fn with_file(&mut self, file: Option<String>) -> &mut Self {
        self.file = file;
        self
    }

    pub fn with_code(&mut self, code: Option<String>) -> &mut Self {
        self.code = code;
        self
    }
}

impl Display for RostError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        if self.code.is_none() {
            panic!("No code supplied for get_error");
        }

        let text = self.code.as_ref().unwrap();

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
            .unwrap_or(self.pos.start);

        let margin: usize = 1;

        let caret = format!(
            "{}{}",
            String::from(" ").repeat(line_pos),
            String::from("^").repeat(self.pos.end - self.pos.start),
        );

        let msg = format!("{}└─ {}", String::from(" ").repeat(line_pos), self.message);

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

        let default_file = "?".to_string();
        let file = self.file.as_ref().unwrap_or(&default_file);
        let header = format!("  --> [{}]:{}:{} => {}", file, line + 1, line_pos + 1, self.kind);

        fmt.write_fmt(format_args!("{}\n{}", header, lines))
    }
}
