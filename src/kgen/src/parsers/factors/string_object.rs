use nom::bytes::complete::escaped;
use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::error::VerboseError;
use nom::IResult;
use nom::sequence::delimited;
use nom_locate::position;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::string_object::StringObject;
use crate::error::error_token::FilePosition;
use crate::parsers::Span;

///
/// Parse an expression which is enclosed in paren.
///
pub fn string_parser(text: Span) -> IResult<Span, EvaluableObject, VerboseError<Span>> {
    let (text, pos) = position(text)?;
    let (text, val) = delimited(
        tag("\""),
        escaped(
            none_of("\\\""),
            '\\',
            tag("\"")
        ),
        tag("\"")
    )(text)?;
    let pos = FilePosition::from_span("File".to_string(), &pos);

    let obj = EvaluableObject::StringObject(Box::new(
            StringObject::new(pos, val.to_string())
        ));

    Ok((text, obj))
}
