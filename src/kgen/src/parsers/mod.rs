use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::map;
use nom::error::VerboseError;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::tuple;
use nom_locate::LocatedSpan;
use crate::ast::functions::FunctionObject;
use crate::parsers::functions::function_parser;

///
/// A text which stores not only a string object but information.
///
pub type Span<'a> = LocatedSpan<&'a str>;

///
/// Parse a whole program.
///
pub fn program_parser(text: Span) -> IResult<Span, Vec<FunctionObject>, VerboseError<Span>> {
    let function_with_space_parser = map(
        tuple((
            space0,
            function_parser,
            space0
        )),
        |(_, function, _)| {
            function
        }
    );

    map(
        tuple((
            many0(function_with_space_parser),
            eof
        )),
        |(functions, _)| {
            functions
        }
    )(text)
}

pub mod exprs;
pub mod factors;
pub mod functions;
pub mod statements;
pub mod utils;
