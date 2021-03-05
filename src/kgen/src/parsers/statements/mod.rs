use nom::error::VerboseError;
use nom::IResult;
use nom::branch::alt;
use crate::ast::statements::StatementObject;
use crate::parsers::Span;
use crate::parsers::statements::assign_object::assign_parser;
use crate::parsers::statements::break_object::break_parser;
use crate::parsers::statements::call_object::call_parser;
use crate::parsers::statements::if_object::if_parser;
use crate::parsers::statements::let_object::let_parser;
use crate::parsers::statements::loop_object::loop_parser;
use crate::parsers::statements::ret_object::ret_parser;

///
/// Parse a statement into `StatementObject`.
///
pub fn statement_parser(text: Span) -> IResult<Span, StatementObject, VerboseError<Span>> {
    alt((
        assign_parser,
        break_parser,
        call_parser,
        if_parser,
        let_parser,
        loop_parser,
        ret_parser
    ))(text)
}

pub mod assign_object;
pub mod break_object;
pub mod call_object;
pub mod if_object;
pub mod let_object;
pub mod loop_object;
pub mod ret_object;
