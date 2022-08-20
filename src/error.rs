use std::{
    fmt::{Display, Formatter},
    ops::Range,
};

pub struct RostErrorElement {
    pos: Range<usize>,
    message: String,
}

impl From<&(String, Range<usize>)> for RostErrorElement {
    fn from((message, pos): &(String, Range<usize>)) -> Self {
        Self {
            pos: pos.clone(),
            message: message.clone(),
        }
    }
}

pub struct RostError {
    kind: String,
    file: Option<String>,
    code: Option<String>,
    elements: Vec<RostErrorElement>,
}

impl RostError {
    pub fn new(kind: String, elements: Vec<RostErrorElement>) -> Self {
        Self {
            elements,
            kind: kind,
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

        let messages = self
            .elements
            .iter()
            .map(|element| {
                let pos = &element.pos;
                let message = &element.message;

                let width = pos.end - pos.start;

                let newlines = text
                    .chars()
                    .enumerate()
                    .take(pos.start)
                    .filter(|&(_, v)| v == '\n')
                    .map(|(i, _)| i);

                let line = newlines.clone().count();
                let line_pos = newlines
                    .clone()
                    .last()
                    .and_then(|i| Some(pos.start - i - 1))
                    .unwrap_or(pos.start);

                (line, line_pos, width, message)
            })
            .collect::<Vec<_>>();

        for (line, line_pos, width, message) in messages {
            let margin: usize = 1;

            let caret = format!(
                "{}{}",
                String::from(" ").repeat(line_pos),
                String::from("^").repeat(width),
            );

            let msg = format!("{}└─ {}", String::from(" ").repeat(line_pos), message);

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
            let header = format!(
                "  --> [{}]:{}:{} => {}",
                file,
                line + 1,
                line_pos + 1,
                self.kind
            );

            fmt.write_fmt(format_args!("{}\n{}", header, lines))?;
        }

        Ok(())
    }
}
