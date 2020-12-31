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

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        let mut resolver =  gen.param_resolver.borrow_mut();
        let param = resolver.find_mut(&self.param_name)
            .ok_or("Unknown parameters".to_string())?;
        let loaded_value = gen.builder.build_load(param.value.into_pointer_value(), "");

        Ok(loaded_value)
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
