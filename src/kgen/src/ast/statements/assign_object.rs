use inkwell::values::BasicValueEnum;
use crate::ast::CodeGen;
use crate::ast::exprs::EvaluableObject;
use crate::error::error_token::{ FilePosition, ErrorToken };

///
/// `AssignObject` is an object which represents a statement which assign some variables.
///
#[derive(Debug,PartialEq)]
pub struct AssignObject {
    pos: FilePosition,
    param_name: String,
    expr: EvaluableObject
}

impl<'ctx> AssignObject {
    ///
    /// Create an `AssignObject` instance.
    ///
    pub fn new(pos: FilePosition, param_name: String, expr: EvaluableObject) -> Self {
        Self {
            pos,
            param_name,
            expr
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        let param_resolver = gen.param_resolver.borrow();

        let param = param_resolver
            .find(&self.param_name)
            .ok_or(
                ErrorToken::error(
                    self.pos.clone(),
                    format!("An unknown parameter named {} is referenced here", self.param_name)
                )
            )?;

        match param.value {
            BasicValueEnum::PointerValue(val) => {
                gen.builder.build_store(val, self.expr.codegen(gen)?);
                Ok(())
            },
            _ => {
                Err(ErrorToken::error(
                    self.pos.clone(),
                    "Cannot assign some value to immutable one".to_string()
                ))
            }
        }
    }
}
