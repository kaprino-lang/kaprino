use std::cell::RefCell;
use inkwell::values::BasicValueEnum;
use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::tuple;
use super::super::program_object::CodeGen;
use super::EvaluableObject;
use super::term_object::term_parser;

#[derive(Debug,PartialEq)]
pub enum ExprOpKind {
    Add,
    Sub
}

#[derive(Debug,PartialEq)]
pub struct ExprObject {
    first: EvaluableObject,
    others: Vec<(ExprOpKind, EvaluableObject)>
}

impl<'ctx> ExprObject {
    pub fn new(first: EvaluableObject, others: Vec<(ExprOpKind, EvaluableObject)>) -> Self {
        Self {
            first, others
        }
    }

    pub fn eval(&self) -> u64 {
        let mut first = self.first.eval();
        for val in &self.others {
            let (op, obj) = val;
            first = match op {
                ExprOpKind::Add => { first + obj.eval() },
                ExprOpKind::Sub => { first - obj.eval() }
            }
        };
        first
    }

    pub fn codegen(&self, gen: &RefCell<CodeGen<'ctx>>) -> Result<BasicValueEnum<'ctx>, &str> {
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
                ExprOpKind::Add => {
                    gen.borrow().builder.build_int_add(left_val, right_val, "")
                },
                ExprOpKind::Sub => {
                    gen.borrow().builder.build_int_sub(left_val, right_val, "")
                }
            }
        };

        Ok(BasicValueEnum::IntValue(left_val))
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
                            '-' => ExprOpKind::Sub,
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
