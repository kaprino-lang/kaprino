use inkwell::values::BasicValueEnum;
use crate::ast::exprs::EvaluableObject;
use crate::error::error_token::{ ErrorToken, FilePosition };
use crate::program_object::CodeGen;

///
/// `FunctionCallObject` is an object which represents an expression which tries to call a function.
///
#[derive(Debug,PartialEq)]
pub struct FunctionCallObject {
    pos: FilePosition,
    func_name: String,
    args: Vec<EvaluableObject>
}

impl<'ctx> FunctionCallObject {
    ///
    /// Create a `FunctionCallObject` instance.
    ///
    pub fn new(pos: FilePosition, func_name: String, args: Vec<EvaluableObject>) -> Self {
        Self {
            pos, func_name, args
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    fn get_args(&self, gen: &CodeGen<'ctx>) -> Result<Vec<BasicValueEnum<'ctx>>, ErrorToken> {
        let mut values: Vec<BasicValueEnum<'ctx>> = Vec::new();

        for arg in &self.args {
            let value = arg.codegen(gen)?;
            values.push(value);
        };

        Ok(values)
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, ErrorToken> {
        let func = gen.module.get_function(&self.func_name)
            .ok_or(
                ErrorToken::error(
                    &self.pos,
                    format!("Not found a function named {}", self.func_name)
                )
            )?;

        let ret_val = gen.builder.build_call(func, self.get_args(gen)?.as_slice(), "funccall");

        ret_val.try_as_basic_value()
            .left()
            .ok_or(
                ErrorToken::error(
                    &self.pos,
                    "Not found a value which will be returned.".to_string()
                )
            )
    }
}
