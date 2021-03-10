use nom::bytes::complete::tag;
use nom::IResult;
use crate::ast::statements::break_object::BreakObject;
use crate::ast::statements::StatementObject;
use crate::parsers::Span;
use crate::parsers::utils::{ get_position, GSError };

///
/// Parse a break statement. Can be written in BNF as follow.
///
/// ```bnf
/// <break> ::= "#break"
/// ```
///
pub fn break_parser(text: Span) -> IResult<Span, StatementObject, GSError> {
    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, _) = tag("#break")(text)?;
    Ok((
        text,
        StatementObject::BreakObject(
            Box::new(
                BreakObject::new(
                    pos
                )
            )
        )
    ))
}
