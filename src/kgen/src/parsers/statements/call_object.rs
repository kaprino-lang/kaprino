use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::IResult;
use crate::ast::statements::call_object::CallObject;
use crate::ast::statements::StatementObject;
use crate::parsers::exprs::expr_parser;
use crate::parsers::Span;
use crate::parsers::utils::{ get_position, GSError };

///
/// Parse a call statement. Can be written in BNF as follow.
///
/// ```bnf
/// <call> ::= "#call" <expr>
/// ```
///
pub fn call_parser(text: Span) -> IResult<Span, StatementObject, GSError> {
    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, _) = tag("#call")(text)?;
    let (text, _) = multispace1(text)?;
    let (text, expr) = expr_parser(text)?;
    Ok((
        text,
        StatementObject::CallObject(
            Box::new(
                CallObject::new(
                    pos,
                    expr
                )
            )
        )
    ))
}
