use inkwell::values::BasicValueEnum;
use crate::ast::exprs::EvaluableObject;
use crate::error::error_token::{ ErrorToken, FilePosition };
use crate::program_object::CodeGen;

///
/// `ExponentsObject` is an object which represents an expression which contains exponentiations.
///
#[derive(Debug,PartialEq)]
pub struct ExponentsObject {
    pos: FilePosition,
    left: EvaluableObject,
    right: EvaluableObject,
}

impl<'ctx> ExponentsObject {
    ///
    /// Create an `ExponentsObject` instance.
    ///
    pub fn new(pos: FilePosition, left: EvaluableObject, right: EvaluableObject) -> Self {
        Self {
            pos, left, right
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    #[allow(unused_variables)]
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, ErrorToken> {
        Err(ErrorToken::error(
            &self.pos,
            "EXPONENTS_OBJECT has not been implemented yet.".to_string()
        ))
    }
}
