use std::fmt::{Arguments, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Element {
    Block(Vec<Element>),
    Raw(String),
    Assign(String),
    Pop,
    Push(String),
    Add,
    Mul,
    Comment(String),
    FunctionCall(String),
    FunctionDefinition {
        identifier: String,
        content: Box<Element>,
    },

    #[allow(dead_code)]
    Pass,
}

impl Element {
    fn fmt(&self, fmt: &mut Formatter<'_>, n_indent: usize) -> Result<(), std::fmt::Error> {
        let indent = "\t".repeat(n_indent);

        let mut w = |args: Arguments| fmt.write_fmt(format_args!("{indent}{args}"));

        match self {
            Element::Block(elements) => {
                for element in elements {
                    element.fmt(fmt, n_indent)?;
                    fmt.write_fmt(format_args!("\n"))?;
                }
            }
            Element::Pass => w(format_args!("pass"))?,
            Element::Assign(name) => w(format_args!("{name} = __intrinsic__stack_pop()"))?,
            Element::Raw(raw) => w(format_args!("{raw}"))?,
            Element::Pop => w(format_args!("__intrinsic__stack_pop()"))?,
            Element::Add => w(format_args!("__intrinsic__stack_add()"))?,
            Element::Mul => w(format_args!("__intrinsic__stack_mul()"))?,
            Element::Comment(line) => w(format_args!("# {line}"))?,
            Element::Push(el) => w(format_args!("__intrinsic__stack_push({el})"))?,
            Element::FunctionCall(identifier) => w(format_args!("{identifier}()"))?,
            Element::FunctionDefinition {
                identifier,
                content,
            } => {
                w(format_args!("def {identifier}():\n"))?;
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
