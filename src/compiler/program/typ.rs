use super::Program;
use std::ops::Range;

pub type TypeId = usize;

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
        declaration_pos: Range<usize>,
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
            TypeKind::Generic { identifier, .. } => identifier.to_string(),
            TypeKind::Intrinsic {
                identifier: typ_identifier,
            } => typ_identifier.into(),
            TypeKind::Function(FunctionTypeKind {
                parameter_type_ids,
                vararg_parameter_type_id,
                ..
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

impl Program {
    pub fn insert_type(&mut self, identifier: Option<&str>, typ: Type) -> usize {
        let type_id = self.types.len();
        self.types.push(typ);

        if let Some(identifier) = identifier {
            self.get_scope_mut()
                .type_lookup
                .insert(identifier.to_string(), type_id);

            self.reverse_type_lookup
                .insert(type_id, identifier.to_string());
        }

        type_id
    }

    pub fn add_intrinsic_type(&mut self, identifier: &str) -> TypeId {
        self.insert_type(
            Some(identifier),
            Type {
                kind: TypeKind::Intrinsic {
                    identifier: identifier.to_string(),
                },
                type_parameters: vec![],
            },
        )
    }

    pub fn add_builtin_types(&mut self) {
        self.add_intrinsic_type("int");
        self.add_intrinsic_type("float");
        self.add_intrinsic_type("str");
        self.add_intrinsic_type("bool");
        self.add_intrinsic_type("any");
        self.add_intrinsic_type("unknown");
    }
}
