use inkwell::values::BasicValueEnum;
use crate::error::error_token::{ ErrorToken, FilePosition };
use crate::ast::CodeGen;

///
/// `ParamObject` is an object which represents a parameter.
///
#[derive(Debug,PartialEq)]
pub struct ParamObject {
    pos: FilePosition,
    param_name: String
}

impl<'ctx> ParamObject {
    ///
    /// Create a `ParamObject` instance.
    ///
    pub fn new(pos: FilePosition, param_name: String) -> Self {
        Self {
            pos, param_name
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, ErrorToken> {
        let mut resolver =  gen.param_resolver.borrow_mut();
        let param = resolver.find_mut(&self.param_name)
            .ok_or(
                ErrorToken::error(
                    self.pos.clone(),
                    "Unknown parameters".to_string()
                )
            )?;
        let loaded_value = gen.builder.build_load(param.value.into_pointer_value(), "");

        Ok(loaded_value)
    }
}
