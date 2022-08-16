use std::fmt::{Display, Formatter};

use super::row::{CodeRow, Row};

#[derive(Debug)]
pub struct Code {
    rows: Vec<CodeRow>,
}

impl Code {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn add(&mut self, row: Row) -> &mut Self {
        self.rows.push(CodeRow::new(row, true));
        self
    }

    pub fn add_with_comment(&mut self, row: Row, comment: String) -> &mut Self {
        self.rows.push(CodeRow::new(row, false));

        self.rows.push(CodeRow::new(Row::Comment(comment), true));
        self
    }
}

impl Display for Code {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in &self.rows {
            fmt.write_fmt(format_args!("{}", row))?
        }

        Ok(())
    }
}
