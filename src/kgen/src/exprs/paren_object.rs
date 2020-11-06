use nom::IResult;
use nom::sequence::tuple;
use nom::combinator::map;
use nom::character::complete::char;
use super::EvaluableObject;
use super::expr_object::expr_parser;

pub fn paren_parser(text: &str) -> IResult<&str, EvaluableObject> {
    map(
        tuple((
            char('('),
            expr_parser,
            char(')')
        )),
        |val| val.1
    )(text)
}
