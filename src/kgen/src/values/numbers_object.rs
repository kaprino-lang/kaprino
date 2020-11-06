use inkwell::values::IntValue;
use nom::IResult;
use nom::combinator::map;
use nom::character::complete::digit1;
use super::EvaluableObject;
use super::super::program_object::CodeGen;

#[derive(Debug,PartialEq)]
pub struct NumberObject {
    number: u64
}

impl NumberObject {
    pub fn new(number: u64) -> NumberObject {
        NumberObject {
            number: number
        }
    }

    pub fn eval(&self) -> u64 {
        self.number
    }

    pub fn codegen<'ctx>(&self, gen: &'ctx CodeGen) -> IntValue<'ctx> {
        let i64_type = gen.context.i64_type();
        let int_val: IntValue<'ctx> = i64_type.const_int(self.number, false);
        int_val
    }
}

pub fn numbers_parser(text: &str) -> IResult<&str, EvaluableObject> {
    map(
        digit1,
        |val: &str| EvaluableObject::NumberObject(
            Box::new(NumberObject::new(val.parse().unwrap()))
        )
    )(text)
}
