use crate::parser::definition::FunctionDeclaration;

use super::builder::Builder;
use std::ops::Range;

// An instruction is an atomic intermediate representation of an instruction

#[derive(Debug)]
pub struct Instruction {
    pub pos: Range<usize>,
    pub kind: InstructionKind,
    pub comment: Option<String>,
}

impl Instruction {
    pub fn new(pos: Range<usize>, kind: InstructionKind) -> Self {
        Self {
            pos,
            kind,
            comment: None,
        }
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }
}

pub type VariableId = usize;
pub type FunctionId = usize;
pub type FunctionTemplateId = usize;
pub type TypeId = usize;
pub type GlobalDataId = usize;

#[derive(Debug)]
pub enum PrimitiveValue {
    Int(i32),
    Float(f32),
    String(String),
}

#[derive(Debug)]
pub struct NormalVariable {
    pub typ: Type,
}

#[derive(Debug)]
pub enum FunctionBody {
    Builtin(String),
    Block {
        body_contains_error: bool,
        content: Builder,
    },
}

#[derive(Debug)]
pub struct FunctionTemplate {
    pub typ: TypeId,
    pub function_declaration: FunctionDeclaration,
}

#[derive(Debug)]
pub struct Function {
    pub function_template_id: FunctionTemplateId,
    pub variable_id: VariableId, // Which variable is this function assigned to?
    pub parameter_variable_ids: Vec<VariableId>,
    pub vararg_parameter: Option<VariableId>,
    pub body: FunctionBody,
}

#[derive(Debug)]
pub enum VariableKind {
    Normal(NormalVariable),
    DeclaredFunction(Function),
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Generic,
    Intrinsic,
    Function {
        parameter_type_ids: Vec<(String, TypeId)>,
        vararg_parameter_type_id: Option<(String, TypeId)>,
        // return_type: TypeId,
    },
    UserDefined {
        identifier: String,
        declaration_pos: Range<usize>,
    },
}

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub type_parameters: Vec<String>, // Todo: add type constraints
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressedType {
    // Todo: impl PartialEq for type inheritance
    // pub identifier: String,
    pub id: TypeId,
    pub arguments: Option<Vec<ExpressedType>>,
}

#[derive(Debug)]
pub struct Variable {
    pub typ: ExpressedType,
    pub declaration_pos: Range<usize>,
    pub assignment_has_error: bool,
    pub identifier: String,
}

#[derive(Debug)]
pub enum ValueKind {
    Primitive(PrimitiveValue),
    Variable(VariableId),
    GlobalData(GlobalDataId),
}

#[derive(Debug)]
pub enum InstructionKind {
    Noop,
    Push(ValueKind),
    Pop,
    Assign(VariableId),
    IntAdd,
    IntMul,
    ProcedureCall(ProcedureCall),
}

// #[derive(Debug)]
// pub struct While {
//     pub condition: Box<Builder>,
//     pub content: Box<Builder>,
// }

// #[derive(Debug)]
// pub struct If {
//     pub condition: Option<Box<Builder>>,
//     pub content: Box<Builder>,
// }

#[derive(Debug, Clone)]
pub enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    Equality,
}

#[derive(Debug)]
pub struct ProcedureCall {
    pub variable_id: VariableId,
    pub nargs: usize,
}
