use std::cell::RefCell;
use inkwell::values::BasicValueEnum;
use nom::combinator::map;
use nom::IResult;
use nom::character::complete::alphanumeric1;
use super::super::program_object::CodeGen;
use super::EvaluableObject;

#[derive(Debug,PartialEq)]
pub struct ParamObject {
    param_name: String
}

impl<'ctx> ParamObject {
    pub fn new(param_name: String) -> Self {
        Self {
            param_name
        }
    }

    pub fn codegen(&self, gen: &RefCell<CodeGen<'ctx>>) -> Result<BasicValueEnum<'ctx>, &str> {
        match gen.borrow_mut().param_resolver.find_mut(&self.param_name) {
            Some(val) => Ok(val.value),
            None => Err("Unknown parameters")
        }
    }
}

pub fn param_parser(text: &str) -> IResult<&str, EvaluableObject> {
    map(
        alphanumeric1,
        |val: &str| {
            EvaluableObject::ParamObject(
                Box::new(
                    ParamObject::new(val.to_string())
                )
            )
        }
    )(text)
}
