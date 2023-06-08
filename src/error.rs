use itertools::Itertools;

use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    ops::Range,
};

static TAB_SIZE: usize = 4;

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
    format!("\x1b[91m{msg}\x1b[0m")
}

#[allow(dead_code)]
fn yellow(msg: &str) -> String {
    format!("\x1b[93m{msg}\x1b[0m")
}

#[allow(dead_code)]
fn green(msg: &str) -> String {
    format!("\x1b[92m{msg}\x1b[0m")
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
            kind,
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
                .map(|i| pos.start - i - 1)
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
            .map(|(line, line_pos, _, _)| format!("{}:{}:{}", file, line + 1, line_pos + 1))
            .join(", ");

        format!("  --> {} => {}\n", positions, self.kind)
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

        let get_ntabs = |line: usize, start: usize, amount: usize| {
            text_lines
                .get(line)
                .unwrap()
                .chars()
                .skip(start)
                .take(amount)
                .filter(|&v| v == '\t')
                .count()
        };

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

        // Get the largest width of line numbers
        let number_padding = (*wanted_row_groups
            .iter()
            .map(|row_group| row_group.iter().max().unwrap_or(&0))
            .max()
            .unwrap_or(&0) as f32
            + 1.0)
            .log10() as usize;

        for (i, row_group) in wanted_row_groups.into_iter().enumerate() {
            if i != 0 {
                writeln!(fmt, "    {}...", " ".repeat(number_padding))?;
            }

            for row_index in row_group {
                let text_line = text_lines.get(row_index).unwrap().to_string();
                let text_line_without_tabs: String = text_line
                    .chars()
                    .map(|v| match v {
                        '\t' => String::from(" ").repeat(TAB_SIZE),
                        _ => v.to_string(),
                    })
                    .collect();

                let padding =
                    " ".repeat(number_padding - (1.0 + row_index as f32).log10() as usize);
                let code_row = format!(
                    "{}{} | {}\n",
                    padding,
                    row_index + 1,
                    text_line_without_tabs
                );
                fmt.write_str(code_row.as_str())?;

                if let Some(messages) = grouped_messages_lookup.get(&row_index) {
                    write!(fmt, "{}  | ", " ".repeat(number_padding))?;

                    let mut positions = messages
                        .iter()
                        .map(|(_, line_pos, width, _)| (line_pos, width))
                        .collect::<Vec<_>>();

                    positions.sort_by(|(a, _), (b, _)| a.cmp(b));
                    let mut acc = 0;
                    for &(line_pos, width) in positions.iter() {
                        let offset = *line_pos - acc;
                        let ntabs = get_ntabs(row_index, acc, *line_pos);

                        fmt.write_fmt(format_args!(
                            "{}{}",
                            String::from(" ").repeat(offset - ntabs + TAB_SIZE * ntabs),
                            red(&String::from("^").repeat(*width)),
                        ))?;

                        acc = *line_pos + *width;
                    }

                    fmt.write_str("\n")?;

                    for (i, &(_, line_pos, _, message)) in messages.iter().rev().enumerate() {
                        write!(fmt, "{}  | ", " ".repeat(number_padding))?;

                        let mut prev_pos = 0;
                        for &(line_pos, width) in positions.iter().take(positions.len() - 1 - i) {
                            let active_pos = *line_pos - prev_pos;
                            let ntabs = get_ntabs(row_index, prev_pos, active_pos);

                            fmt.write_fmt(format_args!(
                                "{}{}{}",
                                String::from(" ").repeat(active_pos - ntabs + TAB_SIZE * ntabs),
                                red("│"),
                                String::from(" ").repeat(*width - 1),
                            ))?;

                            prev_pos = *line_pos + *width;
                        }

                        let offset = line_pos - prev_pos;
                        let ntabs = get_ntabs(row_index, prev_pos, offset);
                        let msg = format!(
                            "{}{} {}",
                            String::from(" ").repeat(offset - ntabs + TAB_SIZE * ntabs),
                            red("└─"),
                            red(message)
                        );
                        fmt.write_fmt(format_args!("{msg}\n"))?;
                    }
                }
            }
        }

        Ok(())
    }
}
