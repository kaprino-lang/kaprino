use nom::IResult;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::param_object::ParamObject;
use crate::parsers::Span;
use crate::parsers::utils::{ identifier, get_position, GSError };

///
/// Parse a parameter.
///
pub fn param_parser(text: Span) -> IResult<Span, EvaluableObject, GSError> {
    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, param) = identifier(text)?;

    let obj = EvaluableObject::ParamObject(Box::new(
            ParamObject::new(pos, param.to_string())
        ));

    Ok((text, obj))
}
