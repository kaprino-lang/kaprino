use nom::IResult;
use nom::branch::alt;
use super::EvaluableObject;
use super::paren_object::paren_parser;
use super::numbers_object::numbers_parser;

pub fn factor_parser(text: &str) -> IResult<&str, EvaluableObject> {
    alt((
        numbers_parser,
        paren_parser
    ))(text)
}
