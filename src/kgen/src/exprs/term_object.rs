use inkwell::values::BasicValueEnum;
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

impl<'ctx> TermObject {
    pub fn new(first: EvaluableObject, others: Vec<(TermOpKind, EvaluableObject)>) -> Self {
        Self {
            first,
            others
        }
    }

    pub fn eval(&self) -> u32 {
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

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        let native_left_val = self.first.codegen(gen)?;

        let mut left_val = match native_left_val {
            BasicValueEnum::IntValue(val) => Ok(val),
            _ => Err("This values cannot be added.")
        }?;

        for val in &self.others {
            let (op, obj) = val;

            let native_right_val = obj.codegen(gen)?;

            let right_val = match native_right_val {
                BasicValueEnum::IntValue(val) => Ok(val),
                _ => Err("This values cannot be added.")
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
