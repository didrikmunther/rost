use crate::{
    compiler::{
        error::CompilerError,
        program::ir::{Instruction, InstructionKind, PrimitiveType, Variable, VariableId},
    },
    parser::definition::VariableDeclaration,
};

use super::{builder::Builder, ir::VariableKind, Program};

/*
    ```rost
    fn main() {
        let a = 1 + 2;
        let b = a + 3 * 4;

        printf("%i %i", a, b);
    }
    ```

    ```irrepresentation
    global_vars = ["%i %i\n"]
    instructions = [
        Push(Int(1), Int),
        Push(Int(2), Int),
        IntAdd,
        Assign(Var(0), Int),
        Push(Int(3), Int),
        Push(Int(4), Int),
        IntMul,
        Assign(Var(1), Int),
        Push(Var(1), Int),
        Push(Var(0), Int),
        Push(GlobalVar(0), String),
        BuiltinFunction("printf", 3),
        Pop,
    ]
    ```
*/

impl Program {
    fn insert_variable(&mut self, variable: Variable) -> VariableId {
        let variable_id = self.variables.len();
        self.variable_lookup
            .insert(variable.name.clone(), variable_id);
        self.variables.push(variable);

        variable_id
    }

    pub fn get_variable(&mut self, identifier: &str) -> Option<VariableId> {
        self.variable_lookup.get(identifier).copied()
    }

    pub fn handle_variable_declaration(
        &mut self,
        declaration: &VariableDeclaration,
    ) -> Result<Builder, CompilerError> {
        let value = self.handle_expression(&declaration.right)?;

        let variable_id = self.insert_variable(Variable {
            name: declaration.identifier.clone(),
            typ: PrimitiveType::Int,
            kind: VariableKind::Local,
        });

        let builder = Builder::new().append(value);

        Ok(builder.push(Instruction {
            pos: declaration.identifier_pos.start..declaration.right_pos.end,
            comment: Some(format!("Assign: {}", declaration.identifier)),
            kind: InstructionKind::Assign(variable_id),
        }))

        // let builder = Builder::new()
        //     .push(
        //         Instruction::new(
        //             declaration.identifier_pos.start..declaration.right_pos.end,
        //             InstructionKind::Assign(variable_id, ValueKind::Int(1)),
        //         )
        //         .with_comment(format!("Variable declaration: {}", declaration.identifier)),
        //     )
        //     .push(Instruction::new(
        //         declaration.identifier_pos.start..declaration.right_pos.end,
        //         InstructionKind::BinaryOperation {
        //             operator: Arithmetic::Add,
        //             left: variable_id,
        //             right: ValueKind::Int(2),
        //         },
        //     ));

        // Ok(builder)

        // todo!()
    }

    // pub fn handle_variable_assignment(
    //     &mut self,
    //     assignment: &VariableAssignment,
    // ) -> Result<Builder, CompilerError> {
    //     let infered_left = self.infer_type(&assignment.left)?;
    //     let infered_right = self.infer_type(&assignment.right)?;

    //     let builder = match &assignment.left.kind {
    //         ExpressionKind::Primary(Primary::Identifier(identifier)) => {
    //             let Some(variable) = self.get_variable(identifier) else {
    //                 return Err(CompilerError::new(
    //                     assignment.left_pos.clone(),
    //                     CompilerErrorKind::UndefinedVariable(identifier.clone()),
    //                 ));
    //             };

    //             if infered_left != infered_right {
    //                 return Err(CompilerError::new(
    //                     assignment.right_pos.clone(),
    //                     CompilerErrorKind::WrongAssignmentType {
    //                         got: infered_right,
    //                         typ: infered_left,
    //                         declaration_pos: Some(variable.pos.clone()),
    //                     },
    //                 ));
    //             }

    //             let location = variable.location.clone();

    //             Builder::new()
    //                 .append(self.handle_expression(&assignment.right)?)
    //                 .push(Procedure {
    //                     pos: assignment.left_pos.start..assignment.right_pos.end,
    //                     comment: Some(format!("Reassign: {identifier}")),
    //                     kind: ProcedureKind::Assign(Assign {
    //                         location,
    //                         size: Self::get_type_size(&infered_right),
    //                     }),
    //                 })
    //         }
    //         ExpressionKind::Unary(Unary {
    //             expr,
    //             operator,
    //             operator_pos: _operator_pos,
    //         }) => {
    //             if *operator != Keyword::Asterix {
    //                 todo!()
    //             }

    //             if infered_left != infered_right {
    //                 return Err(CompilerError::new(
    //                     assignment.right_pos.clone(),
    //                     CompilerErrorKind::WrongAssignmentType {
    //                         got: infered_right,
    //                         typ: infered_left,
    //                         declaration_pos: None,
    //                     },
    //                 ));
    //             }

    //             Builder::new()
    //                 .push(Procedure::new(
    //                     assignment.left_pos.clone(),
    //                     ProcedureKind::Comment(format!("Assignment: {expr:?}")),
    //                 ))
    //                 .append(self.handle_expression(expr)?)
    //                 .append(self.handle_expression(&assignment.right)?)
    //                 .push(Procedure {
    //                     pos: assignment.left_pos.start..assignment.right_pos.end,
    //                     comment: Some("Reassign pointer value".to_string()),
    //                     kind: ProcedureKind::Assign(Assign {
    //                         location: VariableLocation::Address,
    //                         size: Self::get_type_size(&VariableType::Value(Keyword::Int)), // todo: maybe wrong size?
    //                     }),
    //                 })
    //         }
    //         ExpressionKind::MemberAccess(access) => {
    //             let field_type = self.get_struct_field_type(&access.left, &access.member)?;

    //             if infered_left != infered_right {
    //                 return Err(CompilerError::new(
    //                     assignment.right_pos.clone(),
    //                     CompilerErrorKind::WrongAssignmentType {
    //                         got: infered_right,
    //                         typ: infered_left,
    //                         declaration_pos: Some(field_type.pos.clone()),
    //                     },
    //                 ));
    //             }

    //             Builder::new()
    //                 .append(self.handle_member_access_without_deref(&assignment.left, access)?)
    //                 .append(self.handle_expression(&assignment.right)?)
    //                 .push(Procedure {
    //                     pos: assignment.left_pos.start..assignment.right_pos.end,
    //                     comment: Some(format!("Reassign member: {}", access.member)),
    //                     kind: ProcedureKind::Assign(Assign {
    //                         location: VariableLocation::Address,
    //                         size: Self::get_type_size(&infered_right),
    //                     }),
    //                 })
    //         }
    //         _ => {
    //             todo!("Unknown {:?}", assignment.left.kind)
    //         }
    //     };

    //     Ok(builder)
    // }
}
