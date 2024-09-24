use crate::parser::definition::FunctionDeclaration;

use super::{builder::Builder, Program};
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

pub struct TypeIdWithIdentifier {
    pub id: TypeId,
    pub identifier: String,
}

#[derive(Debug, Clone)]
pub struct FunctionTypeKind {
    pub parameter_type_ids: Vec<TypeIdWithIdentifier>,
    pub vararg_parameter_type_id: Option<TypeIdWithIdentifier>,
    pub declaration_pos: Range<usize>,
    // return_type: TypeId,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Generic {
        identifier: String,
    },
    Intrinsic {
        identifier: String,
    },
    Function(FunctionTypeKind),
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

impl Type {
    pub fn format(&self, program: &Program) -> String {
        match &self.kind {
            TypeKind::Generic { identifier } => identifier.to_string(),
            TypeKind::Intrinsic {
                identifier: typ_identifier,
            } => typ_identifier.into(),
            TypeKind::Function(FunctionTypeKind {
                parameter_type_ids,
                vararg_parameter_type_id,
                declaration_pos,
            }) => {
                let parameters = parameter_type_ids
                    .iter()
                    .map(|TypeIdWithIdentifier { id, identifier }| {
                        let typ = program.types.get(*id).unwrap();
                        format!("{}: {}", identifier, typ.format(program))
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                let vararg = vararg_parameter_type_id
                    .as_ref()
                    .map(|TypeIdWithIdentifier { id, identifier }| {
                        let typ = program.types.get(*id).unwrap();
                        format!(", ...{}: {}", identifier, typ.format(program))
                    })
                    .unwrap_or_else(|| "".to_string());

                format!("({}{vararg}) -> todo", parameters)
            }
            TypeKind::UserDefined { identifier, .. } => identifier.to_string(),
        }
    }
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
