use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::tuple;
use super::super::program_object::CodeGen;
use super::super::statements::statement_object::statement_parser;
use super::super::statements::StatementObject;
use super::args_object::args_parser;
use super::function_type_object::function_type_parser;
use super::FunctionInfo;
use super::FunctionObject;
use super::FunctionObjectTrait;

#[derive(Debug,PartialEq)]
pub struct StatementFunction {
    info: FunctionInfo,
    statements: Vec<StatementObject>
}

impl<'ctx> FunctionObjectTrait for StatementFunction {
    fn get_info(&self) -> &FunctionInfo {
        &self.info
    }
}

impl<'ctx> StatementFunction {
    pub fn new(
            func_name: String,
            args: Vec<String>,
            types: Vec<String>,
            ret_type: String,
            statements: Vec<StatementObject>) -> Self {
        let info = FunctionInfo::new(
            func_name, args, types, ret_type
        );
        Self {
            info, statements
        }
    }

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), String> {
        match self.get_func_type(gen) {
            Ok(func_type) => {
                let func = gen.module.add_function(&self.get_info().name, func_type, None);
                let basic_block = gen.context.append_basic_block(func, "entry");

                gen.builder.position_at_end(basic_block);

                self.assign_args(gen, &func);

                for st in &self.statements {
                    st.codegen(gen);
                };

                gen.param_resolver.borrow_mut().remove_scope();

                Ok(())
            },
            Err(error_message) => {
                Err(error_message)
            }
        }
    }
}

pub fn statement_function_parser(text: &str) -> IResult<&str, FunctionObject> {
    map(
        tuple((
            tag("#func"),
            space0,
            alphanumeric1,
            space0,
            args_parser,
            space0,
            function_type_parser,
            space0,
            tag("|>"),
            map(
                many0(
                    tuple((
                        space0,
                        statement_parser,
                        space0
                    ))
                ),
                |val| {
                    let vec: Vec<StatementObject> = val.into_iter().map(|v| { v.1 }).collect();
                    vec
                }
            ),
            tag("|<"),
        )),
        |val| {
            let (_, _, func_name, _, args, _, fn_type, _, _, statements, _) = val;

            let func_name = func_name.to_string();
            let args: Vec<String> = args.iter().map(|s| { s.to_string() }).collect();
            let types: Vec<String> = fn_type.0.iter().map(|s| { s.to_string() }).collect();
            let ret_type = fn_type.1.to_string();

            FunctionObject::StatementFunction(
                Box::new(
                    StatementFunction::new(
                        func_name, args, types, ret_type, statements
                    )
                )
            )
        }
    )(text)
}
