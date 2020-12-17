use super::program_object::CodeGen;
use let_object::LetObject;
use ret_object::RetObject;

#[derive(Debug,PartialEq)]
pub enum StatementObject {
    RetObject(Box<RetObject>),
    LetObject(Box<LetObject>)
}

impl<'ctx> StatementObject {
    pub fn codegen(&self, gen: &CodeGen<'ctx>) {
        match self {
            StatementObject::RetObject(obj) => obj.codegen(gen),
            StatementObject::LetObject(obj) => obj.codegen(gen)
        };
    }
}

pub mod statement_object;
pub mod let_object;
pub mod ret_object;
