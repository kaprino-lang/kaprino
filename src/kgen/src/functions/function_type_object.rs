use nom::IResult;
use nom::sequence::tuple;
use nom::combinator::opt;
use nom::combinator::map;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::character::complete::alphanumeric1;
use super::args_object::args_parser;

pub fn function_type_parser(text: &str) -> IResult<&str, (Vec<&str>, &str)> {
    map(
        tuple((
            args_parser,
            space0,
            tag("->"),
            space0,
            opt(alphanumeric1)
        )),
        |val| {
            let (args, _, _, _, ret) = val;
            match ret {
                Some(ret) => (args, ret),
                None => (args, "")
            }
        }
    )(text)
}
