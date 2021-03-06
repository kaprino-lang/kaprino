use nom::bytes::complete::tag;
use nom::character::complete::{ multispace0 };
use nom::combinator::map;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::tuple;
use crate::ast::statements::loop_object::LoopObject;
use crate::ast::statements::StatementObject;
use crate::parsers::Span;
use crate::parsers::statements::statement_parser;
use crate::parsers::utils::{ get_position, GSError };

///
/// Parse a loop statement. Can be written in BNF as follow.
///
/// ```bnf
/// <loop> ::= "#loop" "|>" <statements> "<|"
/// ```
///
pub fn loop_parser(text: Span) -> IResult<Span, StatementObject, GSError> {
    let statement_with_space_parser = map(
        tuple((
            multispace0,
            statement_parser,
            multispace0
        )),
        |(_, statement, _)| {
            statement
        }
    );

    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, _) = tag("#loop")(text)?;
    let (text, _) = multispace0(text)?;
    let (text, _) = tag("|>")(text)?;
    let (text, statements) = many0(statement_with_space_parser)(text)?;
    let (text, _) = tag("|<")(text)?;
    Ok((
        text,
        StatementObject::LoopObject(
            Box::new(
                LoopObject::new(
                    pos,
                    statements
                )
            )
        )
    ))
}
