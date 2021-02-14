use inkwell::values::BasicValueEnum;
use crate::ast::exprs::exponents_object::ExponentsObject;
use crate::ast::exprs::expr_object::ExprObject;
use crate::ast::exprs::function_call_object::FunctionCallObject;
use crate::ast::exprs::numbers_object::NumberObject;
use crate::ast::exprs::param_object::ParamObject;
use crate::ast::exprs::string_object::StringObject;
use crate::ast::exprs::term_object::TermObject;
use crate::error::error_token::ErrorToken;
use crate::program_object::CodeGen;

///
/// `EvaluableObject` is an object which stores a yet-calculated value.
///
#[derive(Debug,PartialEq)]
pub enum EvaluableObject {
    ExponentsObject(Box<ExponentsObject>),
    ExprObject(Box<ExprObject>),
    FunctionCallObject(Box<FunctionCallObject>),
    NumberObject(Box<NumberObject>),
    ParamObject(Box<ParamObject>),
    StringObject(Box<StringObject>),
    TermObject(Box<TermObject>)
}

impl<'ctx> EvaluableObject {
    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, ErrorToken> {
        match self {
            EvaluableObject::ExponentsObject(obj) => obj.codegen(gen),
            EvaluableObject::ExprObject(obj) => obj.codegen(gen),
            EvaluableObject::FunctionCallObject(obj) => obj.codegen(gen),
            EvaluableObject::NumberObject(obj) => obj.codegen(gen),
            EvaluableObject::ParamObject(obj) => obj.codegen(gen),
            EvaluableObject::StringObject(obj) => obj.codegen(gen),
            EvaluableObject::TermObject(obj) => obj.codegen(gen)
        }
    }
}

pub mod exponents_object;
pub mod expr_object;
pub mod function_call_object;
pub mod numbers_object;
pub mod param_object;
pub mod string_object;
pub mod term_object;
