use nom::branch::alt;
use nom::IResult;
use super::FunctionObject;
use super::expr_function::expr_function_parser;
use super::statement_function::statement_function_parser;
use super::external_function::external_function_parser;

pub fn function_parser(text: &str) -> IResult<&str, FunctionObject> {
    alt((
        expr_function_parser,
        statement_function_parser,
        external_function_parser
    ))(text)
}
