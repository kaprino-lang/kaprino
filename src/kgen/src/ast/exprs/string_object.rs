use inkwell::values::BasicValueEnum;
use crate::error::error_token::{ ErrorToken, FilePosition };
use crate::program_object::CodeGen;

///
/// `StringObject` is an object which stores a text.
///
#[derive(Debug,PartialEq)]
pub struct StringObject {
    pos: FilePosition,
    text: String
}

impl<'ctx> StringObject {
    ///
    /// Create a `StringObject` instance.
    ///
    pub fn new(pos: FilePosition, text: String) -> Self {
        Self {
            pos, text
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, ErrorToken> {
        let global_val = gen.builder.build_global_string_ptr(&self.text, "");

        Ok(BasicValueEnum::PointerValue(global_val.as_pointer_value()))
    }
}
