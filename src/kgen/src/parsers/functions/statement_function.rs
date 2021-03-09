use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::tuple;
use crate::ast::functions::FunctionObject;
use crate::ast::functions::statement_function::StatementFunction;
use crate::parsers::functions::args_parser;
use crate::parsers::functions::function_type_parser;
use crate::parsers::Span;
use crate::parsers::statements::statement_parser;
use crate::parsers::utils::{ identifier, get_position, GSError };

///
/// Parse a C-like function into `FunctionObject`.
///
pub fn statement_function_parser(text: Span) -> IResult<Span, FunctionObject, GSError> {
    let statement_with_spaces_parser = map(
        tuple((
            statement_parser,
            multispace0
        )),
        |(statement, _)| {
            statement
        }
    );

    let (text, pos) = get_position("File".to_string())(text)?;
    let (text, _) = tag("#func")(text)?;
    let (text, _) = multispace0(text)?;
    let (text, function_name) = identifier(text)?;
    let (text, _) = multispace0(text)?;
    let (text, args) = args_parser(text)?;
    let (text, _) = multispace0(text)?;
    let (text, function_type) = function_type_parser(text)?;
    let (text, _) = multispace0(text)?;
    let (text, _) = tag("|>")(text)?;
    let (text, _) = multispace0(text)?;
    let (text, statements) = many0(statement_with_spaces_parser)(text)?;
    let (text, _) = tag("|<")(text)?;

    let function_name = function_name.to_string();
    let args = args.iter().map(|s| { s.to_string() }).collect();
    let (types, ret_type) = function_type;
    let types = types.iter().map(|s| { s.to_string() }).collect();
    let ret_type = ret_type.to_string();

    let obj = FunctionObject::StatementFunction(Box::new(
        StatementFunction::new(pos, function_name, args, types, ret_type, statements)
    ));
    Ok((text, obj))
}
