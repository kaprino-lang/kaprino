use nom::character::complete::digit1;
use nom::error::VerboseError;
use nom::IResult;
use nom_locate::position;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::numbers_object::NumberObject;
use crate::error::error_token::FilePosition;
use crate::parsers::Span;

///
/// Parse a number.
///
pub fn numbers_parser(text: Span) -> IResult<Span, EvaluableObject, VerboseError<Span>> {
    let (text, pos) = position(text)?;
    let (text, digit) = digit1(text)?;
    let pos = FilePosition::from_span("file".to_string(), &pos);

    let obj = EvaluableObject::NumberObject(
        Box::new(NumberObject::new(
            pos,
            digit.fragment().parse().unwrap()
        ))
    );

    Ok((text, obj))
}
