use std::fmt::{Display, Formatter};

use super::row::{CodeRow, Row};

#[derive(Debug)]
pub struct Code {
    pub rows: Vec<CodeRow>,
    pub stack_pos: usize,
}

impl Code {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            stack_pos: 0,
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
        }
    }

    pub fn optimized(&self) -> (Self, usize) {
        let mut code: Code = Code::new();
        let mut prev: Option<&CodeRow> = None;
        let mut removed: usize = 0;

        for i in 0..self.rows.len() {
            let row = self.rows.get(i).unwrap();

            if let Row::Comment(_) = row.row {
                code.add(row.row.clone());

                continue;
            }

            if let Some(prev_row) = prev {
                if let Row::Pop(pop) = &row.row {
                    if let Row::Push(push) = &prev_row.row {
                        let equal = pop.eq(push);
                        code.add(Row::Comment(if equal {
                            format!("Optimized: removed push / pop")
                        } else {
                            format!("Optimized: removed push / pop, added mov")
                        }));

                        if !equal {
                            removed += 1;
                            code.add(Row::Move(pop.clone(), push.clone()));
                        } else {
                            removed += 2;
                        }

                        prev = None;
                        continue;
                    }
                }

                code.add(prev_row.row.clone());
            }

            prev = Some(row);
        }

        if let Some(prev) = prev {
            code.add(prev.row.clone());
        }

        (
            Self {
                rows: code.rows,
                stack_pos: self.stack_pos,
            },
            removed,
        )
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
        F: Fn(&mut Self) -> &mut Self,
    {
        let dummy = self.stack_pos % 2 == 0;

        if dummy {
            self.add_with_comment(Row::Push("0".into()), format!("Dummy alignment"));
            self.stack_pos += 1;
        }

        let this = inner(self);

        if dummy {
            this.add_with_comment(Row::Pop("rdx".into()), format!("Removing dummy alignment"));
            this.stack_pos -= 1;
        }

        this
    }

    pub fn add_with_stack<F: FnOnce(usize) -> Row>(&mut self, row_generator: F) -> &mut Self {
        let row = row_generator(self.stack_pos);
        self.add(row)
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
