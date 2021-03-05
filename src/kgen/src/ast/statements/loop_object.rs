use crate::ast::CodeGen;
use crate::ast::statements::StatementObject;
use crate::error::error_token::{ FilePosition, ErrorToken };

///
/// `LoopObject` is an object which represents a statement with the keyword `#loop`.
///
#[derive(Debug,PartialEq)]
pub struct LoopObject {
    pos: FilePosition,
    statements: Vec<StatementObject>
}

impl<'ctx> LoopObject {
    ///
    /// Create a `LoopObject` instance.
    ///
    pub fn new(pos: FilePosition, statements: Vec<StatementObject>) -> Self {
        Self {
            pos,
            statements
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        let current_val = gen.builder.get_insert_block().unwrap().get_parent().unwrap();
        let loop_block = gen.context.append_basic_block(current_val, "");
        let merged_block = gen.context.append_basic_block(current_val, "");

        gen.builder.build_unconditional_branch(loop_block);

        gen.loop_destinations.borrow_mut().push(merged_block);
        {
            gen.builder.position_at_end(loop_block);
            for statement in &self.statements {
                statement.codegen(gen)?;
            };
            gen.builder.build_unconditional_branch(loop_block);
        }
        gen.loop_destinations.borrow_mut().pop();

        gen.builder.position_at_end(merged_block);
        Ok(())
    }
}
