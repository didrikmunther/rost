use crate::{
    compiler::scope::variable::StructType,
    parser::definition::{Expression, StructConstruction},
};

use super::{
    builder::Builder,
    error::{CompilerError, CompilerErrorKind},
    program::Program,
    scope::variable::VariableType,
};

impl Program {
    pub fn handle_struct_construction(
        &mut self,
        _expression: &Expression,
        sconst: &StructConstruction,
    ) -> Result<Builder, CompilerError> {
        let mut builder = Builder::new();

        let Some(variable) = self.get_variable(&sconst.identifier) else {
            return Err(CompilerError::new(
                sconst.identifier_pos.clone(),
                CompilerErrorKind::UndefinedFunction(sconst.identifier.clone()),
            ));
        };

        let VariableType::Struct(StructType {id, size: _size}) = variable.typ else {
            todo!("Variable is not a struct")
        };

        // println!("{:#?}, {}", variable, id);

        let sdec = self.structs.get(id).unwrap();
        let mut field_content = Vec::with_capacity(sdec.fields.len());
        // println!("{:#?}", sdec);

        for (field_identifier, field_dec) in &sdec.fields {
            let Some(field_const) = sconst.fields.get(field_identifier) else {
                todo!("Missing field: {}", field_identifier);
            };

            if field_dec.typ != self.infer_type(&field_const.expr)? {
                todo!("Field wrong type");
            }

            field_content.push((field_identifier.clone(), field_const));
        }

        for (_field_identifier, field_const) in field_content {
            builder = builder.append(self.handle_expression(&field_const.expr)?);
        }

        Ok(builder)

        // for arg in &fcall.args {
        //     let expr = self.handle_expression(arg)?;
        //     builder = builder.append(expr);
        // }

        // if BUILT_IN.contains(&fcall.identifier.as_str()) {
        //     return Ok(builder.push(Procedure::new(
        //         expression.pos.clone(),
        //         ProcedureKind::SystemCall(SystemCall {
        //             nargs: fcall.args.len(),
        //             identifier: fcall.identifier.clone(),
        //         }),
        //     )));
        // }

        // let Some(variable) = self.get_variable(&fcall.identifier) else {
        //     return Err(CompilerError::new(
        //         fcall.identifier_pos.clone(),
        //         CompilerErrorKind::UndefinedFunction(fcall.identifier.clone()),
        //     ));
        // };

        // let VariableType::Function(function_id) = variable.typ else {
        //     todo!("Variable is not a function")
        // };

        // let function = self.functions.get(function_id).unwrap();

        // if function.parameters.len() != fcall.args.len() {
        //     todo!(
        //         "Wrong number of arguments to function, takes {}, {} was given",
        //         function.parameters.len(),
        //         fcall.args.len()
        //     )
        // }

        // for (par, arg) in function.parameters.iter().zip(&fcall.args) {
        //     let arg_type = self.infer_type(arg)?;
        //     let par_type = self.get_variable_type(&par.typ);

        //     if arg_type != par_type {
        //         return Err(CompilerError::new(
        //             arg.pos.clone(),
        //             CompilerErrorKind::WrongArgumentType {
        //                 parameter: par_type,
        //                 argument: arg_type,
        //                 parameter_pos: par.pos.clone(),
        //             },
        //         ));
        //     }
        // }

        // builder = builder.push(Procedure::new(
        //     expression.pos.clone(),
        //     ProcedureKind::ProcedureCall(ProcedureCall {
        //         function_id,
        //         nargs: fcall.args.len(),
        //         returns: function.return_type.is_some(),
        //     }),
        // ));

        // Ok(builder)
    }
}
