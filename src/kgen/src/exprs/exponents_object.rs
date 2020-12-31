use inkwell::values::BasicValueEnum;
use nom::IResult;
use nom::sequence::tuple;
use nom::combinator::opt;
use nom::combinator::map;
use nom::character::complete::char;
use nom::character::complete::space0;
use super::EvaluableObject;
use super::factor_object::factor_parser;
use super::super::program_object::CodeGen;

#[derive(Debug,PartialEq)]
pub struct ExponentsObject {
    left: EvaluableObject,
    right: EvaluableObject,
}

impl<'ctx> ExponentsObject {
    pub fn new(left: EvaluableObject, right: EvaluableObject) -> Self {
        Self {
            left, right
        }
    }

    pub fn eval(&self) -> u32 {
        u32::pow(self.left.eval(), self.right.eval() as u32)
    }

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        self.left.codegen(gen)
    }
}

pub fn exponents_parser(text: &str) -> IResult<&str, EvaluableObject> {
    map(
        tuple((
            factor_parser,
            space0,
            opt(
                tuple((
                    char('^'),
                    space0,
                    exponents_parser
                ))
            )
        )),
        |val| {
            let (factor, _, exponent) = val;
            match exponent {
                None => factor,
                Some(val) => {
                    EvaluableObject::ExponentsObject(
                        Box::new(
                            ExponentsObject {
                                left: factor,
                                right: val.2
                            }
                        )
                    )
                }
            }
        }
    )(text)
}
