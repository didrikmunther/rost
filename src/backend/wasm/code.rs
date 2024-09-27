use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum WasmValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone)]
pub enum Element {
    Block(Vec<Element>),
    Raw(String),
    Assign(String),
    Pop,
    Push(WasmValue),
    Add,
    Mul,
    Comment(String),
    FunctionCall(String),
    FunctionDefinition {
        identifier: String,
        export: bool,
        content: Box<Element>,
    },
}

impl Element {
    fn fmt(&self, fmt: &mut Formatter<'_>, n_indent: usize) -> Result<(), std::fmt::Error> {
        let indent = "\t".repeat(n_indent);

        macro_rules! w {
            ($($arg:tt)*) => {
                fmt.write_fmt(format_args!("{}{}", indent, format_args!($($arg)*)))
            }
        }

        match self {
            Element::Block(elements) => {
                for element in elements {
                    element.fmt(fmt, n_indent)?;
                    fmt.write_fmt(format_args!("\n"))?;
                }
            }
            Element::Raw(raw) => w!("{raw}")?,
            Element::Comment(line) => w!(";; {line}")?,
            Element::Push(value) => w!("{value}")?,
            Element::Assign(identifier) => w!("local.set ${identifier}")?,
            Element::Pop => w!("drop")?,
            Element::FunctionCall(identifier) => w!("call ${identifier}")?,
            Element::FunctionDefinition {
                identifier,
                export,
                content,
            } => {
                w!("(func ${identifier}\n")?;
                content.fmt(fmt, n_indent + 1)?;
                w!(")\n")?;

                if *export {
                    w!("(export \"{identifier}\" (func ${identifier}))")?;
                }
            }
            _ => todo!("{:?}", self),
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
        self.root_element.fmt(fmt, 1)
    }
}

impl Display for WasmValue {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WasmValue::I32(value) => fmt.write_fmt(format_args!("i32.const {}", value)),
            WasmValue::I64(value) => fmt.write_fmt(format_args!("i64.const {}", value)),
            WasmValue::F32(value) => fmt.write_fmt(format_args!("f32.const {}", value)),
            WasmValue::F64(value) => fmt.write_fmt(format_args!("f64.const {}", value)),
        }
    }
}
