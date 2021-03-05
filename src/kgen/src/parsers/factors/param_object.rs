use nom::error::VerboseError;
use nom::IResult;
use nom_locate::position;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::param_object::ParamObject;
use crate::error::error_token::FilePosition;
use crate::parsers::Span;
use crate::parsers::utils::identifier;

///
/// Parse a parameter.
///
pub fn param_parser(text: Span) -> IResult<Span, EvaluableObject, VerboseError<Span>> {
    let (text, pos) = position(text)?;
    let (text, param) = identifier(text)?;
    let pos = FilePosition::from_span("File".to_string(), &pos);

    let obj = EvaluableObject::ParamObject(Box::new(
            ParamObject::new(pos, param.to_string())
        ));

    Ok((text, obj))
}
