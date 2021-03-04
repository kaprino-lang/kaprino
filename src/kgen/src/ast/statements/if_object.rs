use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;
use crate::ast::CodeGen;
use crate::ast::exprs::EvaluableObject;
use crate::ast::statements::StatementObject;
use crate::error::error_token::{ FilePosition, ErrorToken };

///
/// `IfObject` is an object which represents a statement with the keyword `#let`.
///
#[derive(Debug,PartialEq)]
pub struct IfObject {
    pos: FilePosition,
    expr: EvaluableObject,
    statements: Vec<StatementObject>
}

impl<'ctx> IfObject {
    ///
    /// Create an `IfObject` instance.
    ///
    pub fn new(pos: FilePosition, expr: EvaluableObject, statements: Vec<StatementObject>) -> Self {
        Self {
            pos,
            expr,
            statements
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        let expr = match self.expr.codegen(gen)? {
            BasicValueEnum::IntValue(val) => {
                Ok(val)
            },
            _ => {
                Err(ErrorToken::error(
                    self.pos.clone(),
                    "Cannot pass a non-integer value to an if statement.".to_string()
                ))
            }
        }?;
        let expr = gen.builder.build_int_compare(
            IntPredicate::NE,
            expr,
            gen.context.i32_type().const_int(0, true),
            ""
        );

        let current_val = gen.builder.get_insert_block().unwrap().get_parent().unwrap();
        let then_block = gen.context.append_basic_block(current_val, "");
        let merged_block = gen.context.append_basic_block(current_val, "");
        gen.builder.build_conditional_branch(expr, then_block, merged_block);

        gen.builder.position_at_end(then_block);
        for statement in &self.statements {
            statement.codegen(gen)?;
        };
        gen.builder.build_unconditional_branch(merged_block);

        gen.builder.position_at_end(merged_block);
        gen.builder.build_unreachable();
        Ok(())
    }
}
