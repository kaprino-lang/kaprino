use nom::character::complete::char;
use nom::IResult;
use crate::ast::exprs::EvaluableObject;
use crate::parsers::exprs::expr_parser;
use crate::parsers::Span;
use crate::parsers::utils::GSError;

///
/// Parse an expression which is enclosed in paren.
///
pub fn paren_parser(text: Span) -> IResult<Span, EvaluableObject, GSError> {
    let (text, _ ) = char('(')(text)?;
    let (text, expr) = expr_parser(text)?;
    let (text, _) = char(')')(text)?;

    Ok((text, expr))
}
