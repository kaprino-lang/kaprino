use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::tuple;
use super::super::exprs::EvaluableObject;
use super::super::exprs::expr_object::expr_parser;
use super::super::program_object::CodeGen;
use super::args_object::args_parser;
use super::function_type_object::function_type_parser;
use super::FunctionInfo;
use super::FunctionObject;
use super::FunctionObjectTrait;

#[derive(Debug,PartialEq)]
pub struct ExprFunction {
    info: FunctionInfo,
    expr: EvaluableObject
}

impl<'ctx> FunctionObjectTrait for ExprFunction {
    fn get_info(&self) -> &FunctionInfo {
        &self.info
    }
}

impl<'ctx> ExprFunction {
    pub fn new(
            func_name: String,
            args: Vec<String>,
            types: Vec<String>,
            ret_type: String,
            expr: EvaluableObject) -> Self {
        let info = FunctionInfo::new(
            func_name, args, types, ret_type
        );
        Self {
            info, expr
        }
    }

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), String> {
        let func_type =  self.get_func_type(gen)?;

        let func = gen.module.add_function(&self.get_info().name, func_type, None);
        let basic_block = gen.context.append_basic_block(func, "entry");

        gen.builder.position_at_end(basic_block);

        self.assign_args(gen, &func);

        match self.expr.codegen(gen) {
            Ok(expr) => {
                gen.builder.build_return(Some(&expr));

                gen.param_resolver.borrow_mut().remove_scope();

                Ok(())
            },
            Err(error_message) => {
                gen.param_resolver.borrow_mut().remove_scope();

                Err(error_message)
            }
        }
    }
}

pub fn expr_function_parser(text: &str) -> IResult<&str, FunctionObject> {
    map(
        tuple((
            alphanumeric1,
            space0,
            args_parser,
            space0,
            char('='),
            space0,
            expr_parser,
            space0,
            function_type_parser
        )),
        |val| {
            let (func_name, _, args, _, _, _, expr, _, fn_type) = val;

            let func_name = func_name.to_string();
            let args: Vec<String> = args.iter().map(|s| { s.to_string() }).collect();
            let types: Vec<String> = fn_type.0.iter().map(|s| { s.to_string() }).collect();
            let ret_type = fn_type.1.to_string();

            FunctionObject::ExprFunction(
                Box::new(
                    ExprFunction::new(
                        func_name, args, types, ret_type, expr
                    )
                )
            )
        }
    )(text)
}
