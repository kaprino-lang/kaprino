use nom::bytes::complete::escaped;
use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::IResult;
use nom::sequence::delimited;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::string_object::StringObject;
use crate::parsers::Span;
use crate::parsers::utils::{ get_position, GSError };

///
/// Parse an expression which is enclosed in paren.
///
pub fn string_parser(text: Span) -> IResult<Span, EvaluableObject, GSError> {
    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, val) = delimited(
        tag("\""),
        escaped(
            none_of("\\\""),
            '\\',
            tag("\"")
        ),
        tag("\"")
    )(text)?;

    let obj = EvaluableObject::StringObject(Box::new(
            StringObject::new(pos, val.to_string())
        ));

    Ok((text, obj))
}
