use inkwell::values::IntValue;
use nom::IResult;
use nom::sequence::tuple;
use nom::multi::many0;
use nom::branch::alt;
use nom::combinator::map;
use nom::character::complete::char;
use nom::character::complete::space0;
use super::EvaluableObject;
use super::exponents_object::exponents_parser;
use super::super::program_object::CodeGen;

#[derive(Debug,PartialEq)]
pub enum TermOpKind {
    Mul,
    Div
}

#[derive(Debug,PartialEq)]
pub struct TermObject {
    first: EvaluableObject,
    others: Vec<(TermOpKind, EvaluableObject)>
}

impl TermObject {
    pub fn new(first: EvaluableObject, others: Vec<(TermOpKind, EvaluableObject)>) -> TermObject {
        TermObject { first, others }
    }

    pub fn eval(&self) -> u64 {
        let mut first = self.first.eval();
        for val in &self.others {
            let (op, obj) = val;
            first = match op {
                TermOpKind::Mul => { first * obj.eval() },
                TermOpKind::Div => { first / obj.eval() }
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

pub fn term_parser(text: &str) -> IResult<&str, EvaluableObject> {
    map(
        tuple((
            exponents_parser,
            space0,
            many0(
                map(
                    tuple((
                        alt((char('*'), char('/'))),
                        space0,
                        exponents_parser
                    )),
                    |val| {
                        let (op, _, obj) = val;
                        let op_kind = match op {
                            '*' => TermOpKind::Mul,
                            '/' => TermOpKind::Div,
                            _ => TermOpKind::Mul
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
                EvaluableObject::TermObject(
                    Box::new(
                        TermObject::new(first, others)
                    )
                )
            }
        }
    )(text)
}
