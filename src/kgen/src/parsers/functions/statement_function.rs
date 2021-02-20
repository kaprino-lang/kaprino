use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space0;
use nom::error::VerboseError;
use nom::IResult;
use nom::multi::many0;
use nom_locate::position;
use crate::ast::functions::FunctionObject;
use crate::ast::functions::statement_function::StatementFunction;
use crate::ast::statements::StatementObject;
use crate::error::error_token::FilePosition;
use crate::parsers::functions::args_parser;
use crate::parsers::functions::function_type_parser;
use crate::parsers::Span;
use crate::parsers::statements::statement_parser;

//
// Parse a statement enclosed with spaces.
//
fn statement_with_spaces_parser(text: Span) -> IResult<Span, StatementObject, VerboseError<Span>> {
    let (text, _) = space0(text)?;
    let (text, statement) = statement_parser(text)?;
    let (text, _) = space0(text)?;
    Ok((text, statement))
}

//
// Parse a C-like function into `FunctionObject`.
//
pub fn statement_function_parser(text: Span) -> IResult<Span, FunctionObject, VerboseError<Span>> {
    let (text, pos) = position(text)?;
    let (text, _) = tag("#func")(text)?;
    let (text, _) = space0(text)?;
    let (text, function_name) = alphanumeric1(text)?;
    let (text, _) = space0(text)?;
    let (text, args) = args_parser(text)?;
    let (text, _) = space0(text)?;
    let (text, function_type) = function_type_parser(text)?;
    let (text, _) = space0(text)?;
    let (text, _) = tag("|>")(text)?;
    let (text, statements) = many0(statement_with_spaces_parser)(text)?;
    let (text, _) = tag("|<")(text)?;

    let pos = FilePosition::from_span("File".to_string(), &pos);
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
