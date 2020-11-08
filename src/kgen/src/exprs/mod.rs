use inkwell::values::BasicValueEnum;
use super::program_object::CodeGen;
use exponents_object::ExponentsObject;
use expr_object::ExprObject;
use numbers_object::NumberObject;
use term_object::TermObject;

/// Object which represents evaluable expressions.
#[derive(Debug,PartialEq)]
pub enum EvaluableObject {
    NumberObject(Box<NumberObject>),
    ExponentsObject(Box<ExponentsObject>),
    TermObject(Box<TermObject>),
    ExprObject(Box<ExprObject>)
}

impl EvaluableObject {
    pub fn eval(&self) -> u64 {
        match self {
            EvaluableObject::NumberObject(obj) => obj.eval(),
            EvaluableObject::ExponentsObject(obj) => obj.eval(),
            EvaluableObject::TermObject(obj) => obj.eval(),
            EvaluableObject::ExprObject(obj) => obj.eval()
        }
    }

    pub fn codegen<'ctx>(&self, gen: &'ctx CodeGen) -> Result<BasicValueEnum<'ctx>, &str> {
        match self {
            EvaluableObject::NumberObject(obj) => obj.codegen(&gen),
            EvaluableObject::ExponentsObject(obj) => obj.codegen(&gen),
            EvaluableObject::TermObject(obj) => obj.codegen(&gen),
            EvaluableObject::ExprObject(obj) => obj.codegen(&gen)
        }
    }
}

pub mod numbers_object;
pub mod paren_object;
pub mod factor_object;
pub mod exponents_object;
pub mod term_object;
pub mod expr_object;

pub mod test;
