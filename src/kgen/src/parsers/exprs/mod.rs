use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::error::VerboseError;
use nom::IResult;
use nom::multi::many0;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::expr_object::{ ExprObject, ExprOpKind };
use crate::parsers::exprs::term_object::term_parser;
use crate::parsers::Span;
use crate::parsers::utils::get_position;

///
/// Parse a term with one operator. Can be written in BNF as follow.
///
/// ```bnf
/// <term_with_op> ::= ("+" | "-") <term>
/// ```
///
fn term_with_op_parser(text: Span) -> IResult<Span, (ExprOpKind, EvaluableObject), VerboseError<Span>> {
    let (text, op) = alt((char('+'), char('-')))(text)?;
    let (text, _) = multispace0(text)?;
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
    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, left_value) = term_parser(text)?;
    let (text, _) = multispace0(text)?;
    let (text, right_values) = many0(term_with_op_parser)(text)?;

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
