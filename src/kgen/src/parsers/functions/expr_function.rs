use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::tuple;
use crate::ast::exprs::EvaluableObject;
use crate::ast::functions::expr_function::ExprFunction;
use crate::ast::functions::FunctionObject;
use crate::error::error_token::FilePosition;
use crate::parsers::exprs::expr_parser;
use crate::parsers::functions::args_parser;
use crate::parsers::functions::function_type_parser;
use crate::parsers::Span;
use crate::parsers::utils::{ identifier, get_position, GSError };

///
/// Parse a function which has only one expression.
///
pub fn expr_function_parser(text: Span) -> IResult<Span, FunctionObject, GSError> {
    map(
        tuple((
            get_position("File".to_string()),
            identifier,
            multispace0,
            args_parser,
            multispace0,
            char('='),
            multispace0,
            expr_parser,
            multispace0,
            function_type_parser
        )),
        |(pos, func_name, _, args, _, _, _, expr, _, fn_type): (FilePosition, &str, _, Vec<&str>, _, _, _, EvaluableObject, _, (Vec<&str>, &str))| {
            let func_name = func_name.to_string();
            let args: Vec<String> = args.iter().map(|s| { s.to_string() }).collect();
            let (types, ret_type) = fn_type;
            let types: Vec<String> = types.iter().map(|s| { s.to_string() }).collect();
            let ret_type = ret_type.to_string();

            FunctionObject::ExprFunction(
                Box::new(
                    ExprFunction::new(
                        pos, func_name, args, types, ret_type, expr
                    )
                )
            )
        }
    )(text)
}
