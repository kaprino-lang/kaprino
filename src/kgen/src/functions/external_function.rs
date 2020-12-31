use inkwell::module::Linkage;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::tuple;
use super::super::program_object::CodeGen;
use super::args_object::args_parser;
use super::function_type_object::function_type_parser;
use super::FunctionInfo;
use super::FunctionObject;
use super::FunctionObjectTrait;

#[derive(Debug,PartialEq)]
pub struct ExternalFunction {
    info: FunctionInfo
}

impl<'ctx> FunctionObjectTrait for ExternalFunction {
    fn get_info(&self) -> &FunctionInfo {
        &self.info
    }
}

impl<'ctx> ExternalFunction {
    pub fn new(
            func_name: String,
            args: Vec<String>,
            types: Vec<String>,
            ret_type: String) -> Self {
        let info = FunctionInfo::new(
            func_name, args, types, ret_type
        );
        Self {
            info
        }
    }

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), String> {
        let func_type = self.get_func_type(gen)?;

        gen.module.add_function(&self.get_info().name, func_type, Some(Linkage::External));

        Ok(())
    }
}

pub fn external_function_parser(text: &str) -> IResult<&str, FunctionObject> {
    map(
        tuple((
            tag("#extfunc"),
            space0,
            alphanumeric1,
            space0,
            args_parser,
            space0,
            function_type_parser
        )),
        |val| {
            let (_, _, func_name, _, args, _, fn_type) = val;

            let func_name = func_name.to_string();
            let args: Vec<String> = args.iter().map(|s| { s.to_string() }).collect();
            let types: Vec<String> = fn_type.0.iter().map(|s| { s.to_string() }).collect();
            let ret_type = fn_type.1.to_string();

            FunctionObject::ExternalFunction(
                Box::new(
                    ExternalFunction::new(
                        func_name, args, types, ret_type
                    )
                )
            )
        }
    )(text)
}
