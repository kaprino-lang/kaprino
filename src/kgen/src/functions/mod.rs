use inkwell::types::BasicTypeEnum;
use inkwell::types::FunctionType;
use inkwell::values::FunctionValue;
use super::program_object::CodeGen;
use super::resolvers::parameter_resolver::KParameter;
use expr_function::ExprFunction;
use statement_function::StatementFunction;
use external_function::ExternalFunction;

pub mod args_object;
pub mod function_type_object;
pub mod function_object;

#[derive(Debug,PartialEq)]
pub enum FunctionObject {
    ExprFunction(Box<ExprFunction>),
    StatementFunction(Box<StatementFunction>),
    ExternalFunction(Box<ExternalFunction>)
}

impl<'ctx> FunctionObject {
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), String> {
        match self {
            FunctionObject::ExprFunction(obj) => obj.codegen(gen),
            FunctionObject::StatementFunction(obj) => obj.codegen(gen),
            FunctionObject::ExternalFunction(obj) => obj.codegen(gen)
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct FunctionInfo {
    pub name: String,
    pub args: Vec<String>,
    pub types: Vec<String>,
    pub ret_type: String
}

impl FunctionInfo {
    pub fn new(name: String, args: Vec<String>, types: Vec<String>, ret_type: String) -> Self {
        FunctionInfo{
            name,
            args,
            types,
            ret_type
        }
    }
}

pub trait FunctionObjectTrait {
    fn get_info(&self) -> &FunctionInfo;

    fn get_arg_types<'ctx>(&self, gen: &CodeGen<'ctx>) -> Result<Vec<BasicTypeEnum<'ctx>>, String> {
        let mut vec: Vec<BasicTypeEnum<'ctx>> = Vec::new();
        let mut error_message = "".to_string();
        let type_resolver = gen.type_resolver.borrow();

        for arg_type_name in self.get_info().types.iter() {
            let arg_type = type_resolver.find(arg_type_name);

            match arg_type {
                Some(arg_type) => {
                    let arg_type = arg_type.get_type(gen);
                    vec.push(arg_type);
                },
                None => {
                    error_message = format!(
                        "Unknown types {0} were used in declaration of arguments of the function named {1}.",
                        arg_type_name, self.get_info().name
                    );
                    break;
                }
            };
        };

        if error_message.len() == 0 { Ok(vec) } else { Err(error_message) }
    }

    fn get_ret_type<'ctx>(&self, gen: &CodeGen<'ctx>) -> Result<BasicTypeEnum<'ctx>, String> {
        let type_resolver = gen.type_resolver.borrow();
        let ret_type = type_resolver.find(&self.get_info().ret_type);

        match ret_type {
            Some(ret_type) => {
                Ok(ret_type.get_type(gen))
            },
            None => Err(format!(
                "Unknown types were used in declaration of return value of the function named {}.",
                self.get_info().name
            ))
        }
    }

    fn get_func_type<'ctx>(&self, gen: &CodeGen<'ctx>) -> Result<FunctionType<'ctx>, String> {
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

    fn assign_args<'ctx>(&self, gen: &CodeGen<'ctx>, func: &FunctionValue<'ctx>) {
        let mut param_resolver = gen.param_resolver.borrow_mut();

        param_resolver.add_scope(&self.get_info().name);

        let params = func.get_params();
        for (idx, param_name) in self.get_info().args.iter().enumerate() {
            let allocated = gen.builder.build_alloca(params[idx].get_type(), "");
            gen.builder.build_store(allocated, params[idx]);

            let kparam = KParameter::new(
                self.get_info().args[idx].clone(),
                allocated.into()
            );

            param_resolver.add(param_name, kparam);
        };
    }
}

pub mod expr_function;
pub mod statement_function;
pub mod external_function;

mod test;
