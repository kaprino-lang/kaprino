use nom::branch::alt;
use nom::IResult;
use super::EvaluableObject;
use super::function_call_object::function_call_parser;
use super::numbers_object::numbers_parser;
use super::paren_object::paren_parser;
use super::param_object::param_parser;
use super::string_object::string_parser;

pub fn factor_parser(text: &str) -> IResult<&str, EvaluableObject> {
    alt((
        function_call_parser,
        numbers_parser,
        paren_parser,
        param_parser,
        string_parser
    ))(text)
}
