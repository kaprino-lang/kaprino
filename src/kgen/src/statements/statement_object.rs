use nom::IResult;
use nom::branch::alt;
use super::let_object::let_parser;
use super::ret_object::ret_parser;
use super::StatementObject;

pub fn statement_parser(text: &str) -> IResult<&str, StatementObject> {
    alt((
        ret_parser,
        let_parser
    ))(text)
}
