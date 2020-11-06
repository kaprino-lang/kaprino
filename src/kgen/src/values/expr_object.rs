use inkwell::values::IntValue;
use nom::IResult;
use nom::sequence::tuple;
use nom::multi::many0;
use nom::branch::alt;
use nom::combinator::map;
use nom::character::complete::char;
use nom::character::complete::space0;
use super::EvaluableObject;
use super::term_object::term_parser;
use super::super::program_object::CodeGen;

#[derive(Debug,PartialEq)]
pub enum ExprOpKind {
    Add,
    Minus
}

#[derive(Debug,PartialEq)]
pub struct ExprObject {
    first: EvaluableObject,
    others: Vec<(ExprOpKind, EvaluableObject)>
}

impl ExprObject {
    pub fn new(first: EvaluableObject, others: Vec<(ExprOpKind, EvaluableObject)>) -> ExprObject {
        ExprObject { first, others }
    }

    pub fn eval(&self) -> u64 {
        let mut first = self.first.eval();
        for val in &self.others {
            let (op, obj) = val;
            first = match op {
                ExprOpKind::Add => { first + obj.eval() },
                ExprOpKind::Minus => { first - obj.eval() }
            }
        };
        first
    }

    pub fn codegen<'ctx>(&self, gen: &'ctx CodeGen) -> IntValue<'ctx> {
        let i64_type = gen.context.i64_type();
        let int_val: IntValue<'ctx> = i64_type.const_int(0, false);
        int_val
    }
}

pub fn expr_parser(text: &str) -> IResult<&str, EvaluableObject> {
    map(
        tuple((
            term_parser,
            space0,
            many0(
                map(
                    tuple((
                        alt((char('+'), char('-'))),
                        space0,
                        term_parser
                    )),
                    |val| {
                        let (op, _, obj) = val;
                        let op_kind = match op {
                            '+' => ExprOpKind::Add,
                            '-' => ExprOpKind::Minus,
                            _ => ExprOpKind::Add
                        };
                        (op_kind, obj)
                    }
                )
            )
        )),
        |val| {
            let (first, _, others) = val;
            if others.len() == 0 {
                first
            }
            else {
                EvaluableObject::ExprObject(
                    Box::new(
                        ExprObject::new(first, others)
                    )
                )
            }
        }
    )(text)
}
