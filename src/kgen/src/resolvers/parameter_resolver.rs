use super::dictionary::Dictionary;
use inkwell::values::BasicValueEnum;

#[derive(Debug, PartialEq)]
pub struct KParameter<'ctx> {
    pub type_id: &'ctx str,
    pub value: BasicValueEnum<'ctx>
}

impl<'ctx> KParameter<'ctx> {
    pub fn new(type_id: &'ctx str, value: BasicValueEnum<'ctx>) -> KParameter<'ctx> {
        KParameter { type_id, value }
    }
}

pub type ParameterResolver<'ctx> = Dictionary<'ctx, KParameter<'ctx>>;
