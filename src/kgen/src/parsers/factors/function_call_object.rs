use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::function_call_object::FunctionCallObject;
use crate::parsers::exprs::expr_parser;
use crate::parsers::Span;
use crate::parsers::utils::{ identifier, get_position, GSError };

///
/// Parse a factor which calls a function. Can be written in BNF as follow.
///
/// ```bnf
/// <function_call> ::= .* "(" (<expr> ("," <expr>)*)* ")"
/// ```
///
pub fn function_call_parser(text: Span) -> IResult<Span, EvaluableObject, GSError> {
    let expr_with_spaces =
        map(
            tuple((
                multispace0,
                expr_parser,
                multispace0
            )),
            |(_, statement, _)| {
                statement
            }
        );

    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, function_name) = identifier(text)?;
    let (text, _) = multispace0(text)?;
    let (text, _) = tag("(")(text)?;
    let (text, args) = separated_list0(tag(","), expr_with_spaces)(text)?;
    let (text, _) = tag(")")(text)?;

    let obj = EvaluableObject::FunctionCallObject(Box::new(
        FunctionCallObject::new(pos, function_name.to_string(), args)
    ));
    Ok((text, obj))
}
