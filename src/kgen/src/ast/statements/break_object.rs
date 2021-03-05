use crate::ast::CodeGen;
use crate::error::error_token::{ FilePosition, ErrorToken };

///
/// `BreakObject` is an object which represents a statement with the keyword `#break`.
///
#[derive(Debug,PartialEq)]
pub struct BreakObject {
    pos: FilePosition
}

impl<'ctx> BreakObject {
    ///
    /// Create a `BreakObject` instance.
    ///
    pub fn new(pos: FilePosition) -> Self {
        Self {
            pos
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        if let Some(dest) = gen.loop_destinations.borrow().last() {
            gen.builder.build_unconditional_branch(*dest);

            Ok(())
        }
        else {
            Err(ErrorToken::error(
                self.pos.clone(),
                "Cannot exit a loop while you aren't in loop.".to_string()
            ))
        }
    }
}
