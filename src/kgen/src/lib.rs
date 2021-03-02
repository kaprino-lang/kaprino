extern crate inkwell;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate nom;
extern crate nom_locate;

pub mod ast;
pub mod error;
pub mod exprs;
pub mod functions;
pub mod jit;
pub mod parsers;
pub mod program_object;
pub mod resolvers;
pub mod statements;
