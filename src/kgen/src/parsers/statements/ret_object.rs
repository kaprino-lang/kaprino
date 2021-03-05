use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::error::VerboseError;
use nom::IResult;
use nom_locate::position;
use crate::ast::statements::ret_object::RetObject;
use crate::ast::statements::StatementObject;
use crate::parsers::exprs::expr_parser;
use crate::parsers::Span;
use crate::error::error_token::FilePosition;

///
/// Parse a ret statement.  Can be written in BNF as follow.
///
/// ```bnf
/// <ret> ::= "#ret" <expr>"
/// ```
///
pub fn ret_parser(text: Span) -> IResult<Span, StatementObject, VerboseError<Span>> {
    let (text, pos) = position(text)?;
    let (text, _) = tag("#ret")(text)?;
    let (text, _) = multispace1(text)?;
    let (text, expr) = expr_parser(text)?;
    let pos = FilePosition::from_span("File".to_string(), &pos);
    Ok((
        text,
        StatementObject::RetObject(Box::new(
            RetObject::new(pos, expr)
        ))
    ))
}
