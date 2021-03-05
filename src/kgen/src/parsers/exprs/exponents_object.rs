use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::IResult;
use nom_locate::position;
use crate::ast::exprs::EvaluableObject;
use crate::ast::exprs::exponents_object::ExponentsObject;
use crate::error::error_token::FilePosition;
use crate::parsers::factors::factor_parser;
use crate::parsers::Span;

///
/// Parse a factor with an exponentiation operator. Can be written in BNF as follow.
///
/// ```bnf
/// <factor_with_op> ::= "^" <factor>
/// ```
///
fn factor_with_op_parser(text: Span) -> IResult<Span, EvaluableObject, VerboseError<Span>> {
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
pub fn exponents_parser(text: Span) -> IResult<Span, EvaluableObject, VerboseError<Span>> {
    let (text, pos) = position(text)?;
    let (text, left_value) = factor_parser(text)?;
    let (text, _) = multispace0(text)?;
    let (text, right_value) = opt(factor_with_op_parser)(text)?;
    let pos = FilePosition::from_span("File".to_string(), &pos);
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
