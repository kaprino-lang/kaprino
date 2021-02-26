use inkwell::types::BasicTypeEnum;
use inkwell::types::FunctionType;
use inkwell::values::FunctionValue;
use crate::ast::CodeGen;
use crate::ast::functions::expr_function::ExprFunction;
use crate::ast::functions::external_function::ExternalFunction;
use crate::ast::functions::statement_function::StatementFunction;
use crate::error::error_token::{ ErrorToken, FilePosition };
use crate::resolvers::parameter_resolver::KParameter;

///
/// `FunctionObject` is an object which represents a function of sources.
///
#[derive(Debug,PartialEq)]
pub enum FunctionObject {
    ExprFunction(Box<ExprFunction>),
    ExternalFunction(Box<ExternalFunction>),
    StatementFunction(Box<StatementFunction>)
}

impl<'ctx> FunctionObject {
    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        match self {
            FunctionObject::ExprFunction(obj) => obj.codegen(gen),
            FunctionObject::ExternalFunction(obj) => obj.codegen(gen),
            FunctionObject::StatementFunction(obj) => obj.codegen(gen)
        }
    }
}

///
/// `FunctionInfo` is an object which stores information of a function.
///
#[derive(Debug,PartialEq)]
pub struct FunctionInfo {
    pub name: String,
    pub args: Vec<String>,
    pub types: Vec<String>,
    pub ret_type: String
}

impl FunctionInfo {
    ///
    /// Create a `FunctionInfo` instance.
    ///
    pub fn new(name: String, args: Vec<String>, types: Vec<String>, ret_type: String) -> Self {
        FunctionInfo{
            name,
            args,
            types,
            ret_type
        }
    }
}

///
/// `FunctionObjectTrait` is a trait from which you need to have a class inherit.
///
/// All you have to do to use full features of this trait is just to override `get_info`.
///
pub trait FunctionObjectTrait {
    ///
    /// Get information of this function object.
    ///
    /// When you inherit this trait, you need to implement this function first.
    ///
    fn get_info(&self) -> &FunctionInfo;

    ///
    /// Get a list of types of the arguments.
    ///
    fn get_arg_types<'ctx>(&self, gen: &CodeGen<'ctx>, pos: FilePosition) -> Result<Vec<BasicTypeEnum<'ctx>>, ErrorToken> {
        let mut vec: Vec<BasicTypeEnum<'ctx>> = Vec::new();
        let mut error_message: Option<String> = None;
        let type_resolver = gen.type_resolver.borrow();

        for arg_type_name in self.get_info().types.iter() {
            let arg_type = type_resolver.find(arg_type_name);

            match arg_type {
                Some(arg_type) => {
                    let arg_type = arg_type.get_type(gen);
                    vec.push(arg_type);
                },
                None => {
                    error_message = Some(format!(
                        "Unknown types {0} were used in declaration of arguments of the function named {1}.",
                        arg_type_name, self.get_info().name
                    ));
                    break;
                }
            };
        };

        match error_message {
            Some(error_message) => {
                Err(ErrorToken::error(pos, error_message))
            },
            None => {
                Ok(vec)
            }
        }
    }

    ///
    /// Get a type of the return value.
    ///
    fn get_ret_type<'ctx>(&self, gen: &CodeGen<'ctx>, pos: FilePosition) -> Result<BasicTypeEnum<'ctx>, ErrorToken> {
        let type_resolver = gen.type_resolver.borrow();
        let ret_type = type_resolver
            .find(&self.get_info().ret_type)
            .ok_or(ErrorToken::error(
                pos,
                format!(
                    "Unknown types were used in declaration of return value of the function named {}.",
                    self.get_info().name
                )
            ))?;

        Ok(ret_type.get_type(gen))
    }

    ///
    /// Get a LLVM function type.
    ///
    fn get_func_type<'ctx>(&self, gen: &CodeGen<'ctx>, pos: FilePosition) -> Result<FunctionType<'ctx>, ErrorToken> {
        let arg_types = self.get_arg_types(gen, pos.clone())?;
        let ret_type = self.get_ret_type(gen, pos)?;

        let ret_type = match ret_type {
            BasicTypeEnum::ArrayType(val) => val.fn_type(arg_types.as_slice(), true),
            BasicTypeEnum::FloatType(val) => val.fn_type(arg_types.as_slice(), true),
            BasicTypeEnum::IntType(val) => val.fn_type(arg_types.as_slice(), true),
            BasicTypeEnum::PointerType(val) => val.fn_type(arg_types.as_slice(), true),
            BasicTypeEnum::StructType(val) => val.fn_type(arg_types.as_slice(), true),
            BasicTypeEnum::VectorType(val) => val.fn_type(arg_types.as_slice(), true)
        };

        Ok(ret_type)
    }

    ///
    /// Assign arguments to the function.
    ///
    /// Be careful not to try to get the return value - this function returns nothing.
    ///
    fn assign_args<'ctx>(&self, gen: &CodeGen<'ctx>, func: &FunctionValue<'ctx>) {
        let mut param_resolver = gen.param_resolver.borrow_mut();

        param_resolver.add_scope(&self.get_info().name);

        let params = func.get_params();
        for (idx, param_name) in self.get_info().args.iter().enumerate() {
            let allocated = gen.builder.build_alloca(params[idx].get_type(), "");
            gen.builder.build_store(allocated, params[idx]);

            let param = KParameter::new(
                self.get_info().args[idx].clone(),
                allocated.into()
            );

            param_resolver.add(param_name, param);
        };
    }
}

pub mod expr_function;
pub mod external_function;
pub mod statement_function;
