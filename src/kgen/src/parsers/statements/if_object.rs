use nom::bytes::complete::tag;
use nom::character::complete::{ multispace0, multispace1 };
use nom::combinator::map;
use nom::error::VerboseError;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::tuple;
use crate::ast::statements::if_object::IfObject;
use crate::ast::statements::StatementObject;
use crate::parsers::exprs::expr_parser;
use crate::parsers::Span;
use crate::parsers::statements::statement_parser;
use crate::parsers::utils::get_position;

///
/// Parse an if statement. Can be written in BNF as follow.
///
/// ```bnf
/// <if> ::= "#if" <expr> "|>" <statements> "<|"
/// ```
///
pub fn if_parser(text: Span) -> IResult<Span, StatementObject, VerboseError<Span>> {
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
    let (text, _) = tag("#if")(text)?;
    let (text, _) = multispace1(text)?;
    let (text, expr) = expr_parser(text)?;
    let (text, _) = multispace0(text)?;
    let (text, _) = tag("|>")(text)?;
    let (text, statements) = many0(statement_with_space_parser)(text)?;
    let (text, _) = tag("|<")(text)?;
    Ok((
        text,
        StatementObject::IfObject(
            Box::new(
                IfObject::new(
                    pos,
                    expr,
                    statements
                )
            )
        )
    ))
}
