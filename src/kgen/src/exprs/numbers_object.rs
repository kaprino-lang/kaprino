use std::cell::RefCell;
use inkwell::values::BasicValueEnum;
use inkwell::values::IntValue;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;
use super::super::program_object::CodeGen;
use super::EvaluableObject;

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

    pub fn codegen<'ctx>(&self, gen: &'ctx RefCell<CodeGen>) -> Result<BasicValueEnum<'ctx>, &str> {
        let i64_type = gen.borrow().context.i64_type();
        let int_val: IntValue<'ctx> = i64_type.const_int(self.number, false);
        Ok(BasicValueEnum::IntValue(int_val))
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
