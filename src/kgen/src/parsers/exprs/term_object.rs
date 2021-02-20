use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::space0;
use nom::error::VerboseError;
use nom::IResult;
use nom::multi::many0;
use nom_locate::position;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::term_object::{ TermObject, TermOpKind };
use crate::error::error_token::FilePosition;
use crate::parsers::exprs::exponents_object::exponents_parser;
use crate::parsers::Span;

///
/// Parse an expression which contains exponentiations with one operator. Can be written in BNF as follow.
///
/// ```bnf
/// <exponents_with_op> ::= ("*" | "/") <exponents>
/// ```
///
fn exponents_with_op_parser(text: Span) -> IResult<Span, (TermOpKind, EvaluableObject), VerboseError<Span>> {
    let (text, op) = alt((char('*'), char('/')))(text)?;
    let (text, _) = space0(text)?;
    let (text, exp) = exponents_parser(text)?;

    let op = match op {
        '*' => TermOpKind::Mul,
        '/' => TermOpKind::Div,
        _ => panic!("YOU FOUND A BUG IN RUST NOM!")
    };

    Ok((text, (op, exp)))
}

///
 /// Parse a terminal into `EvaluableObject`. Can be written in BNF as follow.
///
/// ```bnf
/// <term> ::= <exponents> (("*" | "/") <exponents>)*
/// ```
///
pub fn term_parser(text: Span) -> IResult<Span, EvaluableObject, VerboseError<Span>> {
    let (text, pos) = position(text)?;
    let (text, left_value) = exponents_parser(text)?;
    let (text, _) = space0(text)?;
    let (text, right_values) = many0(exponents_with_op_parser)(text)?;

    let pos = FilePosition::from_span("File".to_string(), &pos);

    let obj =
        if right_values.len() == 0 {
            left_value
        }
        else {
            EvaluableObject::TermObject(Box::new(
                TermObject::new(pos, left_value, right_values)
            ))
        };

    Ok((text, obj))
}
