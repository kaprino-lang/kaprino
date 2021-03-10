use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::opt;
use nom::IResult;
use crate::ast::exprs::EvaluableObject;
use crate::ast::statements::let_object::LetObject;
use crate::ast::statements::StatementObject;
use crate::parsers::Span;
use crate::parsers::exprs::expr_parser;
use crate::parsers::utils::{ identifier, get_position, GSError };

///
/// Parse an assign object. Can be written in BNF as follow.
///
/// ```bnf
/// <assign> ::= ":=" <expr>
/// ```
///
fn assign_parser(text: Span) -> IResult<Span, EvaluableObject, GSError> {
    let (text, _) = multispace0(text)?;
    let (text, _) = tag(":=")(text)?;
    let (text, _) = multispace0(text)?;
    let (text, expr) = expr_parser(text)?;
    Ok((text, expr))
}

///
/// Parse a type object. Can be written in BNF as follow.
///
/// ```bnf
/// <type> ::= "(" "<-" .* ")"
/// ```
///
fn type_parser(text: Span) -> IResult<Span, &str, GSError> {
    let (text, _) = multispace0(text)?;
    let (text, _) = tag("(")(text)?;
    let (text, _) = multispace0(text)?;
    let (text, _) = tag("<-")(text)?;
    let (text, _) = multispace0(text)?;
    let (text, type_name) = identifier(text)?;
    let (text, _) = multispace0(text)?;
    let (text, _) = tag(")")(text)?;
    let (text, _) = multispace0(text)?;
    Ok((text, type_name))
}

///
/// Parse a let statement. Can be written in BNF as follow.
///
/// ```bnf
/// <let> ::= "#let" .* (":=" <expr>)* "(" "<-"" .* ")"
/// ```
///
pub fn let_parser(text: Span) -> IResult<Span, StatementObject, GSError> {
    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, _) = tag("#let")(text)?;
    let (text, _) = multispace1(text)?;
    let (text, param_name) = identifier(text)?;
    let (text, assign) = opt(assign_parser)(text)?;
    let (text, type_name) = type_parser(text)?;
    Ok((
        text,
        StatementObject::LetObject(
            Box::new(
                LetObject::new(
                    pos,
                    param_name.to_string(),
                    type_name.to_string(),
                    assign
                )
            )
        )
    ))
}
