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

///
/// `CodeGen` stores information required to compile kaprino and has useful methods.
///
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

        type_resolver.borrow_mut().init_default_types();

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
    /// Parse a program and catch errors if exists.
    ///
    pub fn parse(&self, text: &str) -> Result<Vec<FunctionObject>, Vec<ErrorToken>> {
        let text = Span::new(text);
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

    ///
    /// Parse a program and generate LLVM codes. Also catch errors if exists.
    ///
    /// Please note that `codegen` calls a function `parse` internally.
    ///
    pub fn codegen(&self, text: &str) -> Result<(), Vec<ErrorToken>> {
        let parsed = self.parse(text)?;

        let errors: Vec<ErrorToken> = parsed
            .into_iter()
            .filter_map(|obj| {
                match obj.codegen(self) {
                    Ok(()) => None,
                    Err(err) => Some(err)
                }
            })
            .collect();

        if errors.len() == 0 {
            Ok(())
        }
        else {
            Err(errors)
        }
    }
}

pub mod exprs;
pub mod functions;
pub mod statements;
