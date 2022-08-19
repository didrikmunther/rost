use std::fmt::{Display, Formatter};

use super::row::{CodeRow, Row};

#[derive(Debug)]
pub struct Code {
    rows: Vec<CodeRow>,
    pub stack_pos: usize,
}

impl Code {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            stack_pos: 0,
        }
    }

    fn stack_delta(&mut self, row: &Row) -> i32 {
        match row {
            Row::Pop(_) => -1,
            Row::Push(_) => 1,
            _ => 0,
        }
    }

    fn update_stack(&mut self, delta: i32) {
        self.stack_pos = (self.stack_pos as i32 + delta) as usize;
    }

    pub fn add(&mut self, row: Row) -> &mut Self {
        let stack_delta = self.stack_delta(&row);
        self.rows.push(CodeRow::new(row, true));
        self.update_stack(stack_delta);

        self
    }

    pub fn add_with_stack<F: FnOnce(usize) -> Row>(&mut self, row_generator: F) -> &mut Self {
        let row = row_generator(self.stack_pos);
        self.add(row)
    }

    pub fn add_with_comment(&mut self, row: Row, comment: String) -> &mut Self {
        let stack_delta = self.stack_delta(&row);
        self.rows.push(CodeRow::new(row, false));
        self.rows.push(CodeRow::new(Row::Comment(comment), true));
        self.update_stack(stack_delta);

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
