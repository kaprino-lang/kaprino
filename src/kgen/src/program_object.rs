use std::cell::RefCell;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use super::resolvers::type_resolver::TypeResolver;
use super::resolvers::parameter_resolver::ParameterResolver;
use super::resolvers::function_resolver::FunctionResolver;

#[derive(Debug)]
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub param_resolver: RefCell<ParameterResolver<'ctx>>,
    pub type_resolver: RefCell<TypeResolver<'ctx>>,
    pub function_resolver: RefCell<FunctionResolver<'ctx>>
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
}
