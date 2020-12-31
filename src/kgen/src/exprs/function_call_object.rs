use inkwell::values::BasicValueEnum;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;
use nom::character::complete::alphanumeric1;
use super::super::program_object::CodeGen;
use super::EvaluableObject;
use super::expr_object::expr_parser;

#[derive(Debug,PartialEq)]
pub struct FunctionCallObject {
    func_name: String,
    args: Vec<EvaluableObject>
}

impl<'ctx> FunctionCallObject {
    pub fn new(func_name: String, args: Vec<EvaluableObject>) -> Self {
        Self {
            func_name, args
        }
    }

    fn get_args(&self, gen: &CodeGen<'ctx>) -> Result<Vec<BasicValueEnum<'ctx>>, String> {
        let mut values: Vec<BasicValueEnum<'ctx>> = Vec::new();

        for arg in &self.args {
            let value = arg.codegen(gen)?;
            values.push(value);
        };

        Ok(values)
    }

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        let func = gen.module.get_function(&self.func_name)
            .ok_or(format!("Not found a function named {}", self.func_name))?;

        let ret_val = gen.builder.build_call(func, self.get_args(gen)?.as_slice(), "funccall");

        ret_val.try_as_basic_value()
            .left()
            .ok_or("Not found a value which will be returned.".to_string())
    }
}

pub fn function_call_parser(text: &str) -> IResult<&str, EvaluableObject> {
    map(
        tuple((
            alphanumeric1,
            space0,
            tag("("),
            separated_list0(
                tag(","),
                map(
                    tuple((
                    space0,
                    expr_parser,
                    space0
                    )),
                    |v| {
                        v.1
                    }
                )
            ),
            tag(")")
        )),
        |val| {
            let (func_name, _, _, args, _) = val;

            EvaluableObject::FunctionCallObject(
                Box::new(
                    FunctionCallObject::new(func_name.to_string(), args)
                )
            )
        }
    )(text)
}
