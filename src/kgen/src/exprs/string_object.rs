use inkwell::values::BasicValueEnum;
use nom::IResult;
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::character::complete::none_of;
use nom::bytes::complete::escaped;
use super::EvaluableObject;
use super::super::program_object::CodeGen;

#[derive(Debug,PartialEq)]
pub struct StringObject {
    text: String
}

impl<'ctx> StringObject {
    pub fn new(text: String) -> Self {
        Self {
            text
        }
    }

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        let global_val = gen.builder.build_global_string_ptr(&self.text, "");

        Ok(BasicValueEnum::PointerValue(global_val.as_pointer_value()))
    }
}

pub fn string_parser(text: &str) -> IResult<&str, EvaluableObject> {
    map(
        delimited(
            tag("\""),
            opt(
                escaped(
                    none_of("\\\""),
                    '\\',
                    tag("\"")
                )
            ),
            tag("\"")
        ),
        |val: Option<&str>| {
            EvaluableObject::StringObject(
                Box::new(
                    match val {
                        Some(val) => StringObject::new(val.to_string()),
                        None => StringObject::new("".to_string())
                    }
                )
            )
        }
    )(text)
}
