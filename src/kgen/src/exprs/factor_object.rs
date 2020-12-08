use nom::branch::alt;
use nom::IResult;
use super::EvaluableObject;
use super::numbers_object::numbers_parser;
use super::paren_object::paren_parser;
use super::param_object::param_parser;

pub fn factor_parser(text: &str) -> IResult<&str, EvaluableObject> {
    alt((
        numbers_parser,
        paren_parser,
        param_parser
    ))(text)
}
