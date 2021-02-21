use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::error::VerboseError;
use nom::IResult;
use nom::sequence::tuple;
use nom_locate::position;
use crate::ast::functions::external_function::ExternalFunction;
use crate::ast::functions::FunctionObject;
use crate::error::error_token::FilePosition;
use crate::parsers::functions::args_parser;
use crate::parsers::functions::function_type_parser;
use crate::parsers::Span;

///
/// Parse a function which is declared externally.
///
pub fn external_function_parser(text: Span) -> IResult<Span, FunctionObject, VerboseError<Span>> {
    map(
        tuple((
            position,
            tag("#extern"),
            space0,
            alphanumeric1,
            space0,
            args_parser,
            space0,
            function_type_parser
        )),
        |(pos, _, _, func_name, _, args, _, fn_type)| {
            let pos = FilePosition::from_span("File".to_string(), &pos);
            let func_name = func_name.to_string();
            let args: Vec<String> = args.iter().map(|s| { s.to_string() }).collect();
            let types: Vec<String> = fn_type.0.iter().map(|s| { s.to_string() }).collect();
            let ret_type = fn_type.1.to_string();

            FunctionObject::ExternalFunction(
                Box::new(
                    ExternalFunction::new(
                        pos, func_name, args, types, ret_type
                    )
                )
            )
        }
    )(text)
}
