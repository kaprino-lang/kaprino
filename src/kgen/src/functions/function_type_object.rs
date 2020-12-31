use nom::IResult;
use nom::sequence::tuple;
use nom::combinator::opt;
use nom::combinator::map;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::character::complete::alphanumeric1;
use super::args_object::args_inside_parser;

pub fn function_type_parser(text: &str) -> IResult<&str, (Vec<&str>, &str)> {
    map(
        tuple((
            tag("("),
            space0,
            args_inside_parser,
            space0,
            tag("->"),
            space0,
            opt(alphanumeric1),
            space0,
            tag(")")
        )),
        |val| {
            let (_, _, args, _, _, _, ret, _, _) = val;

            (args, ret.unwrap_or(""))
        }
    )(text)
}
