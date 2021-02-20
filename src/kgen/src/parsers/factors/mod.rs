use nom::branch::alt;
use nom::error::VerboseError;
use nom::IResult;
use crate::ast::exprs::EvaluableObject;
use crate::parsers::factors::function_call_object::function_call_parser;
use crate::parsers::factors::numbers_object::numbers_parser;
use crate::parsers::factors::paren_object::paren_parser;
use crate::parsers::factors::param_object::param_parser;
use crate::parsers::factors::string_object::string_parser;
use crate::parsers::Span;

///
/// Parse a factor into `EvaluableObject`.
///
pub fn factor_parser(text: Span) -> IResult<Span, EvaluableObject, VerboseError<Span>> {
    alt((
        function_call_parser,
        numbers_parser,
        paren_parser,
        param_parser,
        string_parser
    ))(text)
}

pub mod function_call_object;
pub mod numbers_object;
pub mod param_object;
pub mod paren_object;
pub mod string_object;
