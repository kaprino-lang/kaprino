use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::combinator::opt;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::tuple;
use crate::ast::functions::FunctionObject;
use crate::parsers::functions::expr_function::expr_function_parser;
use crate::parsers::functions::external_function::external_function_parser;
use crate::parsers::functions::statement_function::statement_function_parser;
use crate::parsers::Span;
use crate::parsers::utils::{ identifier, GSError };

///
/// Parse a function into `FunctionObject`.
///
pub fn function_parser(text: Span) -> IResult<Span, FunctionObject, GSError> {
    alt((
        expr_function_parser,
        external_function_parser,
        statement_function_parser,
    ))(text)
}

///
/// Parse arguments inside paren.
///
pub fn args_inside_parser(text: Span) -> IResult<Span, Vec<&str>, GSError> {
    let one_arg_parser =
        delimited(
            multispace0,
            identifier,
            multispace0
        );

    separated_list0(
        tag(","),
        one_arg_parser
    )(text)
}

///
/// Parse arguments with paren.
///
pub fn args_parser(text: Span) -> IResult<Span, Vec<&str>, GSError> {
    map(
        tuple((
            tag("("),
            multispace0,
            args_inside_parser,
            multispace0,
            tag(")")
        )),
        |(_, _, args, _, _)| {
            args
        }
    )(text)
}

///
/// Parse a type annotation for a function.
///
pub fn function_type_parser(text: Span) -> IResult<Span, (Vec<&str>, &str), GSError> {
    map(
        tuple((
            tag("("),
            multispace0,
            args_inside_parser,
            multispace0,
            tag("->"),
            multispace0,
            opt(identifier),
            multispace0,
            tag(")")
        )),
        |(_, _, args, _, _, _, ret, _, _): (_, _, Vec<&str>, _, _, _, Option<&str>, _, _)| {
            match ret {
                Some(ret) => {
                    (args, ret)
                },
                None => {
                    (args, "")
                }
            }
        }
    )(text)
}

pub mod expr_function;
pub mod external_function;
pub mod statement_function;
