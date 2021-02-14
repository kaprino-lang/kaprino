use crate::ast::exprs::EvaluableObject;
use crate::error::error_token::{ FilePosition, ErrorToken };
use crate::program_object::CodeGen;

///
/// `RetObject` is an object which represents a statement with the keyword `#ret`.
///
#[derive(Debug,PartialEq)]
pub struct RetObject {
    pos: FilePosition,
    expr: EvaluableObject
}

impl<'ctx> RetObject {
    ///
    /// Create a `RetObject` instance.
    ///
    pub fn new(pos: FilePosition, expr: EvaluableObject) -> Self {
        Self {
            pos, expr
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        let ret_val = self.expr.codegen(gen)?;

        gen.builder.build_return(Some(&ret_val));

        Ok(())
    }
}
