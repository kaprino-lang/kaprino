use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;

#[derive(Debug)]
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>
}

pub struct ProgramObject {
    
}
