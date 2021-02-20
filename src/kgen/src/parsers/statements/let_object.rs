use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::IResult;
use nom_locate::position;
use crate::ast::exprs::EvaluableObject;
use crate::ast::statements::let_object::LetObject;
use crate::ast::statements::StatementObject;
use crate::error::error_token::FilePosition;
use crate::parsers::Span;
use crate::parsers::exprs::expr_parser;

///
/// Parse an assign object. Can be written in BNF as follow.
///
/// ```bnf
/// <assign> ::= ":=" <expr>
/// ```
///
fn assign_parser(text: Span) -> IResult<Span, EvaluableObject, VerboseError<Span>> {
    let (text, _) = space0(text)?;
    let (text, _) = tag(":=")(text)?;
    let (text, _) = space0(text)?;
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
fn type_parser(text: Span) -> IResult<Span, &str, VerboseError<Span>> {
    let (text, _) = space0(text)?;
    let (text, _) = tag("(")(text)?;
    let (text, _) = space0(text)?;
    let (text, _) = tag("<-")(text)?;
    let (text, _) = space0(text)?;
    let (text, type_name) = alphanumeric1(text)?;
    let (text, _) = space0(text)?;
    let (text, _) = tag(")")(text)?;
    let (text, _) = space0(text)?;
    Ok((text, type_name.fragment()))
}

///
/// Parse a let statement.  Can be written in BNF as follow.
///
/// ```bnf
/// <let> ::= "#let" .* (":=" <expr>)* "(" "<-"" .* ")"
/// ```
///
pub fn let_parser(text: Span) -> IResult<Span, StatementObject, VerboseError<Span>> {
    let (text, pos) = position(text)?;
    let (text, _) = tag("#let")(text)?;
    let (text, _) = space1(text)?;
    let (text, param_name) = alphanumeric1(text)?;
    let (text, assign) = opt(assign_parser)(text)?;
    let (text, type_name) = type_parser(text)?;
    let pos = FilePosition::from_span("".to_string(), &pos);
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
