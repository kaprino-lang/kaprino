use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::error::VerboseError;
use nom::IResult;
use nom::multi::separated_list0;
use nom_locate::position;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::function_call_object::FunctionCallObject;
use crate::error::error_token::FilePosition;
use crate::parsers::exprs::expr_parser;
use crate::parsers::Span;
use crate::parsers::utils::identifier;

///
/// Parse an expression with spaces.
///
fn expr_with_spaces(text: Span) -> IResult<Span, EvaluableObject, VerboseError<Span>> {
    let (text, _) = multispace0(text)?;
    let (text, expr) = expr_parser(text)?;
    let (text, _) = multispace0(text)?;
    Ok((text, expr))
}

///
/// Parse a factor which calls a function. Can be written in BNF as follow.
///
/// ```bnf
/// <function_call> ::= .* "(" (<expr> ("," <expr>)*)* ")"
/// ```
///
pub fn function_call_parser(text: Span) -> IResult<Span, EvaluableObject, VerboseError<Span>> {
    let (text, pos) = position(text)?;
    let (text, function_name) = identifier(text)?;
    let (text, _) = multispace0(text)?;
    let (text, _) = tag("(")(text)?;
    let (text, args) = separated_list0(tag(","), expr_with_spaces)(text)?;
    let (text, _) = tag(")")(text)?;
    let pos = FilePosition::from_span("File".to_string(), &pos);

    let obj = EvaluableObject::FunctionCallObject(Box::new(
        FunctionCallObject::new(pos, function_name.to_string(), args)
    ));
    Ok((text, obj))
}
