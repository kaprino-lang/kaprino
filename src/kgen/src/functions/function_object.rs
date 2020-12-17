use inkwell::types::BasicTypeEnum;
use inkwell::types::FunctionType;
use inkwell::values::FunctionValue;
use nom::character::complete::char;
use nom::character::complete::space0;
use nom::character::complete::alphanumeric1;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::tuple;
use super::super::program_object::CodeGen;
use super::super::exprs::EvaluableObject;
use super::super::resolvers::parameter_resolver::KParameter;
use super::super::exprs::expr_object::expr_parser;
use super::super::statements::StatementObject;
use super::args_object::args_parser;
use super::function_type_object::function_type_parser;

#[derive(Debug,PartialEq)]
pub struct FunctionObject {
    func_name: String,
    args: Vec<String>,
    types: Vec<String>,
    ret_type: String,
    expr: Option<EvaluableObject>,
    statements: Option<Vec<StatementObject>>
}

impl<'ctx> FunctionObject {
    pub fn new(
            func_name: String,
            args: Vec<String>,
            types: Vec<String>,
            ret_type: String,
            expr: Option<EvaluableObject>,
            statements: Option<Vec<StatementObject>>) -> Self {
        Self {
            func_name, args, types, ret_type, expr, statements
        }
    }

    fn get_arg_types(&self, gen: &CodeGen<'ctx>) -> Result<Vec<BasicTypeEnum<'ctx>>, String> {
        let mut vec: Vec<BasicTypeEnum<'ctx>> = Vec::new();
        let mut error_message = "".to_string();
        let type_resolver = gen.type_resolver.borrow();

        for arg_type_name in self.types.iter() {
            let arg_type = type_resolver.find(arg_type_name);

            match arg_type {
                Some(arg_type) => {
                    let arg_type = arg_type.get_type(gen);
                    vec.push(arg_type);
                },
                None => {
                    error_message = format!(
                        "Unknown types {0} were used in declaration of arguments of the function named {1}.",
                        arg_type_name, self.func_name
                    );
                    break;
                }
            };
        };

        if error_message.len() == 0 { Ok(vec) } else { Err(error_message) }
    }

    fn get_ret_type(&self, gen: &CodeGen<'ctx>) -> Result<BasicTypeEnum<'ctx>, String> {
        let type_resolver = gen.type_resolver.borrow();
        let ret_type = type_resolver.find(&self.ret_type);

        match ret_type {
            Some(ret_type) => {
                Ok(ret_type.get_type(gen))
            },
            None => Err(format!(
                "Unknown types were used in declaration of return value of the function named {}.",
                self.func_name
            ))
        }
    }

    fn get_func_type(&self, gen: &CodeGen<'ctx>) -> Result<FunctionType<'ctx>, String> {
        match self.get_arg_types(gen) {
            Ok(arg_types) => {
                match self.get_ret_type(gen) {
                    Ok(ret_type) => {
                        Ok(match ret_type {
                            BasicTypeEnum::ArrayType(val) => val.fn_type(arg_types.as_slice(), true),
                            BasicTypeEnum::FloatType(val) => val.fn_type(arg_types.as_slice(), true),
                            BasicTypeEnum::IntType(val) => val.fn_type(arg_types.as_slice(), true),
                            BasicTypeEnum::PointerType(val) => val.fn_type(arg_types.as_slice(), true),
                            BasicTypeEnum::StructType(val) => val.fn_type(arg_types.as_slice(), true),
                            BasicTypeEnum::VectorType(val) => val.fn_type(arg_types.as_slice(), true)
                        })
                    },
                    Err(error_message) => {
                        Err(error_message)
                    }
                }
            },
            Err(error_message) => {
                Err(error_message)
            }
        }
    }

    fn assign_args(&self, gen: &CodeGen<'ctx>, func: &FunctionValue<'ctx>) {
        let mut param_resolver = gen.param_resolver.borrow_mut();

        param_resolver.add_scope(&self.func_name);

        let params = func.get_params();
        for (idx, param_name) in self.args.iter().enumerate() {
            let kparam = KParameter::new(
                self.args[idx].clone(),
                params[idx]
            );

            param_resolver.add(param_name, kparam);
        };
    }

    fn codegen_expr(&self, gen: &CodeGen<'ctx>, expr: &EvaluableObject) -> Result<(), String> {
        match self.get_func_type(gen) {
            Ok(func_type) => {
                let func = gen.module.add_function(&self.func_name, func_type, None);
                let basic_block = gen.context.append_basic_block(func, "entry");

                gen.builder.position_at_end(basic_block);

                self.assign_args(gen, &func);

                match expr.codegen(gen) {
                    Ok(expr) => {
                        gen.builder.build_return(Some(&expr));

                        gen.param_resolver.borrow_mut().remove_scope();

                        Ok(())
                    },
                    Err(error_message) => {
                        gen.param_resolver.borrow_mut().remove_scope();

                        Err(error_message.to_string())
                    }
                }
            },
            Err(error_message) => {
                Err(error_message)
            }
        }
    }

    fn codegen_statements(&self, gen: &CodeGen<'ctx>, statements: &Vec<StatementObject>) -> Result<(), String> {
        match self.get_func_type(gen) {
            Ok(func_type) => {
                let func = gen.module.add_function(&self.func_name, func_type, None);
                let basic_block = gen.context.append_basic_block(func, "entry");

                gen.builder.position_at_end(basic_block);

                self.assign_args(gen, &func);

                for st in statements {
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

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), String> {
        match &self.expr {
            Some(expr) => self.codegen_expr(gen, expr),
            None => {
                match &self.statements {
                    Some(statements) => self.codegen_statements(gen, statements),
                    None => Err("Not found expressions or statements.".to_string())
                }
            }
        }
    }
}

pub fn function_parser(text: &str) -> IResult<&str, Box<FunctionObject>> {
    map(
        tuple((
            alphanumeric1,
            space0,
            char('('),
            space0,
            args_parser,
            space0,
            char(')'),
            space0,
            char('='),
            space0,
            expr_parser,
            space0,
            char('('),
            space0,
            function_type_parser,
            space0,
            char(')')
        )),
        |val| {
            let (func_name, _, _, _, args, _, _, _, _, _, expr, _, _, _, fn_type, _, _) = val;

            let func_name = func_name.to_string();
            let args: Vec<String> = args.iter().map(|s| { s.to_string() }).collect();
            let types: Vec<String> = fn_type.0.iter().map(|s| { s.to_string() }).collect();
            let ret_type = fn_type.1.to_string();

            Box::new(
                FunctionObject::new(
                    func_name, args, types, ret_type, Some(expr), None
                )
            )
        }
    )(text)
}
