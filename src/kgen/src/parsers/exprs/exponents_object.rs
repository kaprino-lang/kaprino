use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::IResult;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::exponents_object::ExponentsObject;
use crate::parsers::factors::factor_parser;
use crate::parsers::Span;
use crate::parsers::utils::{ get_position, GSError };

///
/// Parse a factor with an exponentiation operator. Can be written in BNF as follow.
///
/// ```bnf
/// <factor_with_op> ::= "^" <factor>
/// ```
///
fn factor_with_op_parser(text: Span) -> IResult<Span, EvaluableObject, GSError> {
    let (text, _) = char('^')(text)?;
    let (text, _) = multispace0(text)?;
    let (text, factor) = factor_parser(text)?;
    Ok((text, factor))
}

///
/// Parse an expression which contains exponentiations into `EvaluableObject`. Can be written in BNF as follow.
///
/// ```bnf
/// <exponents> ::= <factor> ("^" <factor>)*
/// ```
///
pub fn exponents_parser(text: Span) -> IResult<Span, EvaluableObject, GSError> {
    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, left_value) = factor_parser(text)?;
    let (text, _) = multispace0(text)?;
    let (text, right_value) = opt(factor_with_op_parser)(text)?;
    let obj = match right_value {
        None => {
            left_value
        },
        Some(right_value) => {
            EvaluableObject::ExponentsObject(Box::new(
                ExponentsObject::new(pos, left_value, right_value)
            ))
        }
    };
    Ok((text, obj))
}
