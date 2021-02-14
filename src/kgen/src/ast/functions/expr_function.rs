use crate::ast::exprs::EvaluableObject;
use crate::ast::functions::{ FunctionInfo, FunctionObjectTrait };
use crate::error::error_token::{ ErrorToken, FilePosition };
use crate::program_object::CodeGen;

///
/// `ExprFunction` is an object which represents a function declared with a mathematical style, like `f(x) = x * x`.
///
#[derive(Debug,PartialEq)]
pub struct ExprFunction {
    pos: FilePosition,
    info: FunctionInfo,
    expr: EvaluableObject
}

impl<'ctx> FunctionObjectTrait for ExprFunction {
    ///
    /// Get information of this function object.
    ///
    fn get_info(&self) -> &FunctionInfo {
        &self.info
    }
}

impl<'ctx> ExprFunction {
    ///
    /// Create an `ExprFunction` instance.
    ///
    pub fn new(
            pos: FilePosition,
            func_name: String,
            args: Vec<String>,
            types: Vec<String>,
            ret_type: String,
            expr: EvaluableObject) -> Self {
        let info = FunctionInfo::new(
            func_name, args, types, ret_type
        );
        Self {
            pos, info, expr
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        let func_type =  self.get_func_type(gen, &self.pos)?;

        let func = gen.module.add_function(&self.get_info().name, func_type, None);
        let basic_block = gen.context.append_basic_block(func, "entry");

        gen.builder.position_at_end(basic_block);

        self.assign_args(gen, &func);

        match self.expr.codegen(gen) {
            Ok(expr) => {
                gen.builder.build_return(Some(&expr));

                gen.param_resolver.borrow_mut().remove_scope();

                Ok(())
            },
            Err(error_message) => {
                gen.param_resolver.borrow_mut().remove_scope();

                Err(error_message)
            }
        }
    }
}
