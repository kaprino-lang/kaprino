use nom::character::complete::digit1;
use nom::IResult;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::numbers_object::NumberObject;
use crate::parsers::Span;
use crate::parsers::utils::{ get_position, GSError };

///
/// Parse a number.
///
pub fn numbers_parser(text: Span) -> IResult<Span, EvaluableObject, GSError> {
    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, digit) = digit1(text)?;

    let obj = EvaluableObject::NumberObject(
        Box::new(NumberObject::new(
            pos,
            digit.fragment().parse().unwrap_or(0)
        ))
    );

    Ok((text, obj))
}
