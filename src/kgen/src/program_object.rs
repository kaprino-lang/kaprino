use std::cell::RefCell;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::tuple;
use super::functions::function_object::function_parser;
use super::functions::FunctionObject;
use super::resolvers::function_resolver::FunctionResolver;
use super::resolvers::parameter_resolver::ParameterResolver;
use super::resolvers::type_resolver::TypeResolver;

#[derive(Debug)]
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub param_resolver: RefCell<ParameterResolver<'ctx>>,
    pub type_resolver: RefCell<TypeResolver<'ctx>>,
    pub function_resolver: RefCell<FunctionResolver<'ctx>>
}

fn program_parser(text: &str) -> IResult<&str, Vec<FunctionObject>> {
    map(
        tuple((
            many0(
                tuple((
                    space0,
                    function_parser,
                    space0
                ))
            ),
            eof
        )),
        |(val, _)| {
            val.into_iter().map(|v| { v.1 }).collect()
        }
    )(text)
}

impl<'ctx> CodeGen<'_> {
    pub fn new(context: &'ctx Context, name: &str) -> CodeGen<'ctx> {
        let module = context.create_module(name);
        let builder = context.create_builder();
        let param_resolver = RefCell::new(ParameterResolver::new());
        let type_resolver = RefCell::new(TypeResolver::new());
        let function_resolver = RefCell::new(FunctionResolver::new());

        CodeGen {
            context,
            module,
            builder,
            param_resolver,
            type_resolver,
            function_resolver
        }
    }

    pub fn init(&self) {
        self.type_resolver.borrow_mut().init_default_types();
    }

    pub fn parse(&self, text: &str) -> Result<(), String> {
        let parsed = program_parser(text);
        match parsed {
            Ok((_, parsed)) => {
                for obj in parsed {
                    let result = obj.codegen(self);

                    if let Err(text) = result {
                        return Err(text);
                    };
                };

                Ok(())
            },
            Err(text) => {
                Err(text.to_string())
            }
        }
    }
}
