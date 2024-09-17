use std::fmt::{Arguments, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Element {
    Block(Vec<Element>),
    Raw(String),
    Push(String),
    FunctionCall {
        identifier: String,
        nargs: usize,
    },
    FunctionDefinition {
        name: String,
        params: Vec<String>,
        content: Box<Element>,
    },
}

impl Element {
    fn fmt(&self, fmt: &mut Formatter<'_>, n_indent: usize) -> Result<(), std::fmt::Error> {
        let mut w = |args: Arguments| fmt.write_fmt(args);

        let indent = "\t".repeat(n_indent);

        match self {
            Element::Block(elements) => {
                for element in elements {
                    element.fmt(fmt, n_indent)?;
                    fmt.write_fmt(format_args!("\n"))?;
                }
            }
            Element::Raw(raw) => {
                w(format_args!("{indent}{raw}"))?;
            }
            Element::Push(el) => {
                w(format_args!("{indent}__intrinsic__stack_push({el})"))?;
            }
            Element::FunctionCall { identifier, nargs } => {
                w(format_args!(
                    "{indent}__intrinsic__stack_callf({identifier}, {nargs})"
                ))?;
            }
            Element::FunctionDefinition {
                name,
                params,
                content,
            } => {
                let params = params.join(", ");
                w(format_args!("{indent}def {name}({params}):\n"))?;
                content.fmt(fmt, n_indent + 1)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Code {
    pub root_element: Element,
}

impl Code {
    pub fn new(root_element: Element) -> Self {
        Self { root_element }
    }
}

impl Display for Code {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.root_element.fmt(fmt, 0)
    }
}
