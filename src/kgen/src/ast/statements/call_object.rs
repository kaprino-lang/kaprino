use crate::ast::CodeGen;
use crate::ast::exprs::EvaluableObject;
use crate::error::error_token::{ FilePosition, ErrorToken };

///
/// `CallObject` is an object which represents a statement with the keyword `#call`.
///
#[derive(Debug,PartialEq)]
pub struct CallObject {
    pos: FilePosition,
    expr: EvaluableObject
}

impl<'ctx> CallObject {
    ///
    /// Create a `CallObject` instance.
    ///
    pub fn new(pos: FilePosition, expr: EvaluableObject) -> Self {
        Self {
            pos,
            expr
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        self.expr.codegen(gen)?;
        Ok(())
    }
}
