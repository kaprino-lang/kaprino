use crate::program_object::CodeGen;
use crate::ast::functions::{ FunctionInfo, FunctionObjectTrait };
use crate::ast::statements::StatementObject;
use crate::error::error_token::{ ErrorToken, FilePosition };

///
/// `StatementFunction` is an object which represents a function declared in C-like style.
///
#[derive(Debug,PartialEq)]
pub struct StatementFunction {
    pos: FilePosition,
    info: FunctionInfo,
    statements: Vec<StatementObject>
}

impl<'ctx> FunctionObjectTrait for StatementFunction {
    ///
    /// Get information of this function object.
    ///
    fn get_info(&self) -> &FunctionInfo {
        &self.info
    }
}

impl<'ctx> StatementFunction {
    ///
    /// Create a `StatementFunction` instance.
    ///
    pub fn new(
            pos: FilePosition,
            func_name: String,
            args: Vec<String>,
            types: Vec<String>,
            ret_type: String,
            statements: Vec<StatementObject>) -> Self {
        let info = FunctionInfo::new(
            func_name, args, types, ret_type
        );
        Self {
            pos, info, statements
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        let func_type = self.get_func_type(gen, &self.pos)?;

        let func = gen.module.add_function(&self.get_info().name, func_type, None);
        let basic_block = gen.context.append_basic_block(func, "entry");

        gen.builder.position_at_end(basic_block);

        self.assign_args(gen, &func);

        for st in &self.statements {
            st.codegen(gen)?;
        };

        gen.param_resolver.borrow_mut().remove_scope();

        Ok(())
    }
}
