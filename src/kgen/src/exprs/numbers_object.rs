use inkwell::values::BasicValueEnum;
use inkwell::values::IntValue;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;
use super::super::program_object::CodeGen;
use super::EvaluableObject;

#[derive(Debug,PartialEq)]
pub struct NumberObject {
    number: u32
}

impl<'ctx> NumberObject {
    pub fn new(number: u32) -> Self{
        Self {
            number
        }
    }

    pub fn eval(&self) -> u32 {
        self.number
    }

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        let i32_type = gen.context.i32_type();
        let int_val: IntValue<'ctx> = i32_type.const_int(self.number.into(), false);
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
