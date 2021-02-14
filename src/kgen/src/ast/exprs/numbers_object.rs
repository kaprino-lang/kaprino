use inkwell::values::{ BasicValueEnum, IntValue };
use crate::error::error_token::{ ErrorToken, FilePosition };
use crate::program_object::CodeGen;

///
/// `NumberObject` is an object which stores a number.
///
#[derive(Debug,PartialEq)]
pub struct NumberObject {
    pos: FilePosition,
    number: u32
}

impl<'ctx> NumberObject {
    ///
    /// Create a `NumberObject` instance.
    ///
    pub fn new(pos: FilePosition, number: u32) -> Self{
        Self {
            pos, number
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, ErrorToken> {
        let i32_type = gen.context.i32_type();
        let int_val: IntValue<'ctx> = i32_type.const_int(self.number.into(), false);
        Ok(BasicValueEnum::IntValue(int_val))
    }
}
