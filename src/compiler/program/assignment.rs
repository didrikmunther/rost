use crate::{
    compiler::{
        error::CompilerError,
        program::ir::{
            Arithmetic, Instruction, InstructionKind, PrimitiveType, ValueKind, Variable,
            VariableId,
        },
    },
    parser::definition::VariableDeclaration,
};

use super::{builder::Builder, Program};

/*
    fn main() {
        let a = 1 + 2 + 3 + 4;
        let b = a + 3 * 4;

        printf("%i %i", a, b);
    }
    ```

    ```irrepresentation
    global_vars = ["%i %i\n"]
    instructions = [
        Assign(1, Int(1)),
        Add(1, Int(2)),
        Add(1, Int(3)),
        Add(1, Int(4)),


        Assign(2, Int(3)),
        Mul(2, Int(4)),
        Assign(3, Var(1)),
        Add(3, Var(2)),

        
        Function("printf", [GlobalVar(0), Var(1), Var(3)]),
    ]
```
*/

impl Program {
    fn insert_variable(&mut self, variable: Variable) -> VariableId {
        self.variables.push(variable);
        self.variables.len() - 1
    }

    pub fn handle_variable_declaration(
        &mut self,
        declaration: &VariableDeclaration,
    ) -> Result<Builder, CompilerError> {
        let variable_id = self.insert_variable(Variable {
            name: Some(declaration.identifier.clone()),
            typ: PrimitiveType::Int,
        });

        let builder = Builder::new()
            .push(
                Instruction::new(
                    declaration.identifier_pos.start..declaration.right_pos.end,
                    InstructionKind::Assign(variable_id, ValueKind::Int(1)),
                )
                .with_comment(format!("Variable declaration: {}", declaration.identifier)),
            )
            .push(Instruction::new(
                declaration.identifier_pos.start..declaration.right_pos.end,
                InstructionKind::BinaryOperation {
                    operator: Arithmetic::Add,
                    left: variable_id,
                    right: ValueKind::Int(2),
                },
            ));

        Ok(builder)
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
