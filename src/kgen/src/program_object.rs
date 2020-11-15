use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use super::resolvers::type_resolver::TypeResolver;
use super::resolvers::parameter_resolver::ParameterResolver;

#[derive(Debug)]
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub param_resolver: ParameterResolver<'ctx>,
    pub type_resolver: TypeResolver<'ctx>
}

impl<'ctx> CodeGen<'_> {
    pub fn new(context: &'ctx Context, name: &str) -> CodeGen<'ctx> {
        let module = context.create_module(name);
        let builder = context.create_builder();
        let param_resolver = ParameterResolver::new();
        let type_resolver = TypeResolver::new();

        CodeGen { context, module, builder, param_resolver, type_resolver }
    }
}
