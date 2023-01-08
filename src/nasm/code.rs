use std::fmt::{Display, Formatter};

use super::row::{CodeRow, Row};

#[derive(Debug)]
pub struct Code {
    pub rows: Vec<CodeRow>,
    pub stack_pos: usize,
    pub function_start_pos: Option<usize>, // If we are in a function, where is the starting position?
}

impl Code {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            stack_pos: 0,
            function_start_pos: None,
        }
    }

    pub fn strip_comments(&mut self) -> Self {
        Self {
            rows: self
                .rows
                .iter()
                .filter(|v| match v.row {
                    Row::Comment(_) => false,
                    _ => true,
                })
                .map(|v| v.clone())
                .collect(),
            stack_pos: self.stack_pos,
            function_start_pos: None,
        }
    }

    pub fn optimized(&self) -> (Self, usize) {
        let mut code: Code = Code::new();
        let mut prev: Option<&CodeRow> = None;

        for i in 0..self.rows.len() {
            let row = self.rows.get(i).unwrap();

            if let Row::Comment(_) = row.row {
                code.add(row.row.clone());

                continue;
            }

            match &row.row {
                Row::Subtract(_, v) | Row::Add(_, v) => {
                    if v.eq("0") {
                        code.add(Row::Comment("Optimized: removed sub/add 0".into()));
                        continue;
                    }
                },
                Row::Move(a, b) => {
                    if a.eq(b) {
                        code.add(Row::Comment("Optimized: removed identical moves".into()));
                        continue;
                    }
                }
                _ => {}
            }

            if let Some(prev_row) = prev {
                match (&prev_row.row, &row.row) {
                    (Row::Push(push), Row::Pop(pop)) => {
                        if pop.eq(push) {
                            code.add(Row::Comment("Optimized: removed push / pop".into()));
                        } else {
                            code.add_with_comment(
                                Row::Move(pop.clone(), push.clone()),
                                "Optimized: removed push / pop, added mov".into(),
                            );
                        }

                        prev = None;
                        continue;
                    }
                    (Row::Move(a1, a2), Row::Move(b1, b2)) => {
                        if a1 == b2 {
                            code.add_with_comment(
                                Row::Move(b1.clone(), a2.clone()),
                                "Optimized: removed mov / mov, added mov".into(),
                            );
                            prev = None;
                            continue;
                        }
                    }
                    // Todo: ADD _, 0
                    // Todo: SUB _, 0
                    _ => {}
                }

                code.add(prev_row.row.clone());
            }

            prev = Some(row);
        }

        if let Some(prev) = prev {
            code.add(prev.row.clone());
        }

        let removed = self.instruction_len() - code.instruction_len();

        (
            Self {
                rows: code.rows,
                stack_pos: self.stack_pos,
                function_start_pos: self.function_start_pos,
            },
            removed,
        )
    }

    /// Gives the amount of instructions without comments and labels
    fn instruction_len(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| match row.row {
                Row::Comment(_) | Row::Label(_) => false,
                _ => true,
            })
            .count()
    }

    fn update_stack(&mut self, delta: isize) {
        self.stack_pos = (self.stack_pos as isize + delta) as usize;
    }

    pub fn add(&mut self, row: Row) -> &mut Self {
        let stack_delta = row.stack_delta();
        self.rows.push(CodeRow::new(row, true));
        self.update_stack(stack_delta);

        self
    }

    /// Aligns the stack such that RSP % 16 == 8.
    /// This is done by pushing and removing a temporary dummy stack element if needed.
    pub fn aligned<F>(&mut self, inner: F) -> &mut Self
    where
        F: FnOnce(&mut Self) -> &mut Self,
    {
        self.add(Row::Comment("Aligning stack to 16".into()))
            .add(Row::Move("rax".into(), "rsp".into()))
            .add(Row::And("rsp".into(), "-16".into()))
            .add(Row::Subtract("rsp".into(), "8".into()))
            .add(Row::Push("rax".into()));

        inner(self);

        self.add(Row::Pop("rsp".into()))
    }

    pub fn add_with_comment(&mut self, row: Row, comment: String) -> &mut Self {
        let stack_delta = row.stack_delta();
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
