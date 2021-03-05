use crate::ast::CodeGen;
use crate::ast::statements::break_object::BreakObject;
use crate::ast::statements::call_object::CallObject;
use crate::ast::statements::if_object::IfObject;
use crate::ast::statements::let_object::LetObject;
use crate::ast::statements::loop_object::LoopObject;
use crate::ast::statements::ret_object::RetObject;
use crate::error::error_token::ErrorToken;

///
/// `StatementObject` is an object which represents a statement of sources.
///
#[derive(Debug,PartialEq)]
pub enum StatementObject {
    BreakObject(Box<BreakObject>),
    CallObject(Box<CallObject>),
    IfObject(Box<IfObject>),
    LetObject(Box<LetObject>),
    LoopObject(Box<LoopObject>),
    RetObject(Box<RetObject>)
}

impl<'ctx> StatementObject {
    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        match self {
            StatementObject::BreakObject(obj) => obj.codegen(gen),
            StatementObject::CallObject(obj) => obj.codegen(gen),
            StatementObject::IfObject(obj) => obj.codegen(gen),
            StatementObject::LetObject(obj) => obj.codegen(gen),
            StatementObject::LoopObject(obj) => obj.codegen(gen),
            StatementObject::RetObject(obj) => obj.codegen(gen)
        }
    }
}

pub mod break_object;
pub mod call_object;
pub mod if_object;
pub mod let_object;
pub mod loop_object;
pub mod ret_object;
