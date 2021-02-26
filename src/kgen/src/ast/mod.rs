use std::cell::RefCell;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use crate::ast::functions::FunctionObject;
use crate::error::error_token::{ErrorToken, FilePosition};
use crate::parsers::program_parser;
use crate::parsers::Span;
use crate::resolvers::function_resolver::FunctionResolver;
use crate::resolvers::parameter_resolver::ParameterResolver;
use crate::resolvers::type_resolver::TypeResolver;

#[derive(Debug)]
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub param_resolver: RefCell<ParameterResolver<'ctx>>,
    pub type_resolver: RefCell<TypeResolver<'ctx>>,
    pub function_resolver: RefCell<FunctionResolver<'ctx>>
}

impl<'ctx> CodeGen<'ctx> {
    ///
    /// Create a `CodeGen` instance.
    ///
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

    ///
    /// Create a `CodeGen` instance.
    ///
    pub fn init(&self) {
        self.type_resolver.borrow_mut().init_default_types();
    }

    ///
    /// Parse a program.
    ///
    pub fn parse(&'ctx self, text: &'ctx str) -> Result<Vec<FunctionObject>, Vec<ErrorToken>> {
        let text: Span<'ctx> = Span::new(text);
        let parsed = program_parser(text);
        match parsed {
            Ok((_, parsed)) => {
                Ok(parsed)
            },
            Err(text) => {
                match text {
                    nom::Err::Error(text) | nom::Err::Failure(text) => {
                        let errors =
                            text.errors
                                .into_iter()
                                .map(|(err, _)| {
                                    let pos = FilePosition::from_span(
                                        "File".to_string(),
                                        &err
                                    );
                                    ErrorToken::error(
                                        pos,
                                        "Syntax error was found.".to_string()
                                    )
                                })
                                .collect();
                        Err(errors)
                    },
                    _ => {
                        Ok(Vec::new())
                    }
                }
            }
        }
    }
}

pub mod exprs;
pub mod functions;
pub mod statements;
