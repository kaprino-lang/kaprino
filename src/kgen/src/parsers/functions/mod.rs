use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::tuple;
use crate::ast::functions::FunctionObject;
use crate::parsers::functions::expr_function::expr_function_parser;
use crate::parsers::functions::external_function::external_function_parser;
use crate::parsers::functions::statement_function::statement_function_parser;
use crate::parsers::Span;

///
/// Parse a function into `FunctionObject`.
///
pub fn function_parser(text: Span) -> IResult<Span, FunctionObject, VerboseError<Span>> {
    alt((
        expr_function_parser,
        external_function_parser,
        statement_function_parser,
    ))(text)
}

///
/// Parse arguments inside paren.
///
pub fn args_inside_parser(text: Span) -> IResult<Span, Vec<&str>, VerboseError<Span>> {
    let second_args_parser =
        map(
            tuple((
                space0,
                tag(","),
                space0,
                alphanumeric1
            )),
            |(_, _, _, arg): (_, _, _, Span)| {
                arg.fragment() as &str
            }
        );

    let internal_args_parser =
        map(
            tuple((
                alphanumeric1,
                many0(second_args_parser)
            )),
            |(first_arg, second_args): (Span, Vec<&str>)| {
                let mut second_args = second_args.clone();
                second_args.insert(0, first_arg.fragment());
                second_args
            }
        );

    let (text, args) = opt(internal_args_parser)(text)?;

    match args {
        Some(args) => Ok((text, args)),
        None => Ok((text, Vec::new()))
    }
}

///
/// Parse arguments with paren.
///
pub fn args_parser(text: Span) -> IResult<Span, Vec<&str>, VerboseError<Span>> {
    map(
        tuple((
            tag("("),
            space0,
            args_inside_parser,
            space0,
            tag(")")
        )),
        |(_, _, args, _, _)| {
            args
        }
    )(text)
}

//
// Parse a type annotation for a function.
//
pub fn function_type_parser(text: Span) -> IResult<Span, (Vec<&str>, &str), VerboseError<Span>> {
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
        |(_, _, args, _, _, _, ret, _, _): (_, _, Vec<&str>, _, _, _, Option<Span>, _, _)| {
            match ret {
                Some(ret) => {
                    (args, ret.fragment() as &str)
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
