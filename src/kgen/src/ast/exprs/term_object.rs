use inkwell::values::BasicValueEnum;
use crate::ast::exprs::EvaluableObject;
use crate::error::error_token::{ ErrorToken, FilePosition };
use crate::program_object::CodeGen;

///
/// `TermOpKind` is an object which represents an operator, `*` or `/`.
///
#[derive(Debug,PartialEq)]
pub enum TermOpKind {
    Mul,
    Div
}

///
/// `TermObject` is an object which represents an expression which contains `*` or `/`.
///
#[derive(Debug,PartialEq)]
pub struct TermObject {
    pos: FilePosition,
    first: EvaluableObject,
    others: Vec<(TermOpKind, EvaluableObject)>
}

impl<'ctx> TermObject {
    ///
    /// Create a `TermObject` instance.
    ///
    pub fn new(pos: FilePosition, first: EvaluableObject, others: Vec<(TermOpKind, EvaluableObject)>) -> Self {
        Self {
            pos, first, others
        }
    }

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, ErrorToken> {
        let native_left_val = self.first.codegen(gen)?;

        let mut left_val = match native_left_val {
            BasicValueEnum::IntValue(val) => Ok(val),
            _ => Err(ErrorToken::error(
                &self.pos,
                "Two values cannot be multiplied.".to_string()
            ))
        }?;

        for val in &self.others {
            let (op, obj) = val;

            let native_right_val = obj.codegen(gen)?;

            let right_val = match native_right_val {
                BasicValueEnum::IntValue(val) => Ok(val),
                _ => Err(ErrorToken::error(
                    &self.pos,
                    "Two values cannot be multiplied.".to_string()
                ))
            }?;

            left_val = match op {
                TermOpKind::Mul => {
                    gen.builder.build_int_mul(left_val, right_val, "")
                },
                TermOpKind::Div => {
                    gen.builder.build_int_unsigned_div(left_val, right_val, "")
                }
            }
        };

        Ok(BasicValueEnum::IntValue(left_val))
    }
}
