use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::space0;
use nom::error::VerboseError;
use nom::IResult;
use nom::multi::many0;
use nom_locate::position;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::expr_object::{ ExprObject, ExprOpKind };
use crate::error::error_token::FilePosition;
use crate::parsers::exprs::term_object::term_parser;
use crate::parsers::Span;

///
/// Parse a term with one operator. Can be written in BNF as follow.
///
/// ```bnf
/// <term_with_op> ::= ("+" | "-") <term>
/// ```
///
fn term_with_op_parser(text: Span) -> IResult<Span, (ExprOpKind, EvaluableObject), VerboseError<Span>> {
    let (text, op) = alt((char('+'), char('-')))(text)?;
    let (text, _) = space0(text)?;
    let (text, term) = term_parser(text)?;

    let op = match op {
        '+' => ExprOpKind::Add,
        '-' => ExprOpKind::Sub,
        _ => panic!("YOU FOUND A BUG IN RUST NOM!")
    };

    Ok((text, (op, term)))
}

///
/// Parse an expression into `EvaluableObject`. Can be written in BNF as follow.
///
/// ```bnf
/// <expr> ::= <term> (("+" | "-") <term>)*
/// ```
///
pub fn expr_parser(text: Span) -> IResult<Span, EvaluableObject, VerboseError<Span>> {
    let (text, pos) = position(text)?;
    let (text, left_value) = term_parser(text)?;
    let (text, _) = space0(text)?;
    let (text, right_values) = many0(term_with_op_parser)(text)?;

    let pos = FilePosition::from_span("File".to_string(), &pos);

    if right_values.len() == 0 {
        Ok((text, left_value))
    }
    else {
        Ok((
            text,
            EvaluableObject::ExprObject(
                Box::new(
                    ExprObject::new(pos, left_value, right_values)
                )
            )
        ))
    }
}

pub mod exponents_object;
pub mod term_object;
