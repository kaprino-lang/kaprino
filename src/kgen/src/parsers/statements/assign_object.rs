use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::IResult;
use crate::ast::statements::assign_object::AssignObject;
use crate::ast::statements::StatementObject;
use crate::parsers::exprs::expr_parser;
use crate::parsers::Span;
use crate::parsers::utils::{ identifier, get_position, GSError };

///
/// Parse an assign statement. Can be written in BNF as follow.
///
/// ```bnf
/// <assign> ::= .* ":=" <expr>
/// ```
///
pub fn assign_parser(text: Span) -> IResult<Span, StatementObject, GSError> {
    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, param_name) = identifier(text)?;
    let (text, _) = multispace0(text)?;
    let (text, _) = tag(":=")(text)?;
    let (text, _) = multispace0(text)?;
    let (text, expr) = expr_parser(text)?;
    Ok((
        text,
        StatementObject::AssignObject(
            Box::new(
                AssignObject::new(
                    pos,
                    param_name.to_string(),
                    expr
                )
            )
        )
    ))
}
