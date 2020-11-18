use super::dictionary::Dictionary;
use inkwell::values::FunctionValue;

#[derive(Debug, PartialEq)]
pub struct KFunction<'ctx> {
    pub type_id: &'ctx str,
    pub func: FunctionValue<'ctx>
}

impl<'ctx> KFunction<'ctx> {
    pub fn new(type_id: &'ctx str, func: FunctionValue<'ctx>) -> KFunction<'ctx> {
        KFunction { type_id, func }
    }
}

pub type FunctionResolver<'ctx> = Dictionary<'ctx, KFunction<'ctx>>;
