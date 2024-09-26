use crate::{
    compiler::{
        error::{CompilerError, CompilerErrorKind},
        program::{
            builder::Builder,
            ir::{Instruction, InstructionKind, ValueKind},
            Program,
        },
    },
    parser::definition::Expression,
};

impl Program {
    /// Pushes the value of the identifier to the stack.
    pub fn handle_identifier(
        &mut self,
        expression: &Expression,
        identifier: &str,
        _load_address: bool,
    ) -> Result<Builder, CompilerError> {
        let Some(variable_id) = self.get_variable(identifier) else {
            return CompilerErrorKind::UndefinedVariable(identifier.to_string())
                .at_pos(&expression.pos)
                .into();
        };

        // let variable = self.variables.get(variable_id).unwrap();
        // let typ = self.types.get(variable.typ.id).unwrap();
        // if let TypeKind::Function(_) = &typ.kind {
        //     self.insert_variable(
        //         Variable {
        //             typ: variable.typ.clone(),
        //             declaration_pos: expression.pos.clone(),
        //             assignment_has_error: false,
        //             identifier: identifier.to_string(),
        //         },
        //         Some(identifier),
        //     );

        //     self.get_scope_mut()
        //         .function_template_lookup
        //         .insert(identifier.to_string(), variable_id);

        //     eprintln!("Defined function: {}", identifier);

        //     return Ok(Builder::new());
        // };

        Ok(Builder::new().push(Instruction::new(
            expression.pos.clone(),
            InstructionKind::Push(ValueKind::Variable(variable_id)),
        )))
    }
}
