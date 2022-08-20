use itertools::Itertools;

use std::{
    collections::HashMap,
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

fn red(msg: &str) -> String {
    format!("\x1b[91m{}\x1b[0m", msg)
}

#[allow(dead_code)]
fn yellow(msg: &str) -> String {
    format!("\x1b[93m{}\x1b[0m", msg)
}

#[allow(dead_code)]
fn green(msg: &str) -> String {
    format!("\x1b[92m{}\x1b[0m", msg)
}

pub struct RostError {
    kind: String,
    file: Option<String>,
    code: Option<String>,
    elements: Vec<RostErrorElement>,
    margin: usize,
}

impl RostError {
    pub fn new(kind: String, elements: Vec<RostErrorElement>) -> Self {
        Self {
            elements,
            kind: kind,
            file: None,
            code: None,
            margin: 1,
        }
    }

    #[allow(dead_code)]
    pub fn with_file(&mut self, file: Option<String>) -> &mut Self {
        self.file = file;
        self
    }

    #[allow(dead_code)]
    pub fn with_code(&mut self, code: Option<String>) -> &mut Self {
        self.code = code;
        self
    }

    #[allow(dead_code)]
    pub fn with_margin(&mut self, margin: usize) -> &mut Self {
        self.margin = margin;
        self
    }

    fn get_messages(&self) -> impl Iterator<Item = (usize, usize, usize, &String)> + Clone {
        let text = self.code.as_ref().unwrap();

        self.elements.iter().map(|element| {
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
    }

    fn get_header<'a>(
        &self,
        messages: impl Iterator<Item = (usize, usize, usize, &'a String)>,
    ) -> String {
        let default_file = &"?".to_string();
        let file = self.file.as_ref().unwrap_or(default_file);
        let positions = messages
            .map(|(line, line_pos, _, _)| format!("{}:{}", line + 1, line_pos + 1))
            .join(", ");

        format!("  --> [{}]:{{{}}} => {}\n", file, positions, self.kind)
    }
}

impl Display for RostError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        if self.code.is_none() {
            panic!("No code supplied for get_error");
        }

        let text = self.code.as_ref().unwrap();
        let messages = self.get_messages();
        fmt.write_str(&self.get_header(messages.clone()))?;
        let text_lines = text
            .lines()
            .chain(std::iter::once("")) // lines() ignores a potential last whitespace line, add it manually
            .collect::<Vec<&str>>();

        let mut grouped_messages = messages
            .clone()
            .group_by(|(line, _, _, _)| *line)
            .into_iter()
            .map(|(gi, group)| (gi, group.into_iter().collect()))
            .collect::<Vec<(usize, Vec<_>)>>();

        grouped_messages.sort_by(|(gi1, _), (gi2, _)| gi1.cmp(gi2));
        let grouped_messages_lookup: HashMap<_, _> = grouped_messages.into_iter().collect();

        // Rows of code
        let total_lines = text.chars().filter(|&c| c == '\n').count() + 1;
        let mut wanted_rows = messages
            .flat_map(|(line, _, _, _)| {
                ((line as i32 - self.margin as i32).clamp(0, total_lines as i32 - 1) as usize)
                    ..=(line + self.margin).clamp(0, total_lines - 1)
            })
            .collect::<Vec<_>>();

        wanted_rows.sort();
        wanted_rows.dedup();

        let wanted_row_groups = wanted_rows
            .into_iter()
            .enumerate()
            .group_by(|(i, v)| v - i)
            .into_iter()
            .map(|v| v.1.into_iter().map(|(_, v)| v).collect())
            .collect::<Vec<Vec<usize>>>();

        for (i, row_group) in wanted_row_groups.into_iter().enumerate() {
            if i != 0 {
                fmt.write_str("    ...\n")?;
            }

            for row_index in row_group {
                let text_line = text_lines.get(row_index).unwrap().to_string();
                let code_row = format!("{} | {}\n", row_index + 1, text_line);
                fmt.write_str(code_row.as_str())?;

                if let Some(messages) = grouped_messages_lookup.get(&row_index) {
                    fmt.write_str("  | ")?;

                    let mut positions = messages
                        .into_iter()
                        .map(|(_, line_pos, width, _)| (line_pos, width))
                        .collect::<Vec<_>>();

                    positions.sort_by(|(_, a), (_, b)| a.cmp(b));
                    let mut acc = 0;
                    for &(line_pos, width) in positions.iter() {
                        fmt.write_fmt(format_args!(
                            "{}{}",
                            String::from(" ").repeat(*line_pos - acc),
                            String::from("^").repeat(*width)
                        ))?;
                        acc += *line_pos + 1;
                    }

                    fmt.write_str("\n")?;

                    for (i, &(_, line_pos, _, message)) in messages.into_iter().rev().enumerate() {
                        fmt.write_str("  | ")?;

                        let mut pipes = 0;
                        for &(position, _) in positions.iter().rev().skip(i + 1) {
                            pipes += 1 + *position;
                            fmt.write_fmt(format_args!(
                                "{}│",
                                String::from(" ").repeat(*position)
                            ))?;
                        }

                        let msg = format!(
                            "{}└─ {}",
                            String::from(" ").repeat(line_pos - pipes),
                            red(message)
                        );
                        fmt.write_fmt(format_args!("{}\n", msg))?;
                    }
                }
            }
        }

        Ok(())
    }
}
