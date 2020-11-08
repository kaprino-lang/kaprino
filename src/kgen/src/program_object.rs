use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;

#[derive(Debug)]
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>
}

impl CodeGen<'_> {
    pub fn new<'ctx>(context: &'ctx Context, name: &str) -> CodeGen<'ctx> {
        let module = context.create_module(name);
        let builder = context.create_builder();

        CodeGen { context, module, builder }
    }
}
