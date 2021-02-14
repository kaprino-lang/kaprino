use inkwell::module::Linkage;
use crate::ast::functions::{ FunctionInfo, FunctionObjectTrait };
use crate::error::error_token::{ ErrorToken, FilePosition };
use crate::program_object::CodeGen;

///
/// `ExternalFunction` is an object which represents a function implemented in external resources.
///
#[derive(Debug,PartialEq)]
pub struct ExternalFunction {
    pos: FilePosition,
    info: FunctionInfo
}

impl<'ctx> FunctionObjectTrait for ExternalFunction {
    ///
    /// Get information of this function object.
    ///
    fn get_info(&self) -> &FunctionInfo {
        &self.info
    }
}

impl<'ctx> ExternalFunction {
    ///
    /// Create an `ExternalFunction` instance.
    ///
    pub fn new(
            pos: FilePosition,
            func_name: String,
            args: Vec<String>,
            types: Vec<String>,
            ret_type: String) -> Self {
        let info = FunctionInfo::new(
            func_name, args, types, ret_type
        );
        Self {
            pos, info
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        let func_type = self.get_func_type(gen, &self.pos)?;

        gen.module.add_function(&self.get_info().name, func_type, Some(Linkage::External));

        Ok(())
    }
}
