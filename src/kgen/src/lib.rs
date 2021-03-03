extern crate inkwell;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate nom;
extern crate nom_locate;

pub mod ast;
pub mod error;
pub mod jit;
pub mod parsers;
pub mod resolvers;
