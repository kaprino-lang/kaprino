use nom::{ IResult, Slice };
use nom::error::{ ErrorKind, ParseError };
use nom_locate::position;
use nom_greedyerror::GreedyError;
use regex::Regex;
use crate::error::error_token::FilePosition;
use crate::parsers::Span;

///
/// `GSError` is an initial of Greedy Span Error. Can be used for tracking an error with `alt` correctly.
///
/// For more information, please refer [nom-greedyerror](https://github.com/dalance/nom-greedyerror), an awesome project.
///
pub type GSError<'a> = GreedyError<Span<'a>, ErrorKind>;

lazy_static! {
    ///
    /// A regex expression for specifying identifiers.
    ///
    pub static ref IDENTIFIER_REGEX: Regex = {
        Regex::new(r"[a-zA-Z_]([a-zA-Z0-9_])*(!)*").unwrap()
    };
}

///
/// Parse a word which is valid for the identifiers of kaprino.
///
pub fn identifier(content: Span) -> IResult<Span, &str, GSError> {
    let matched = IDENTIFIER_REGEX
        .captures(content.fragment())
        .ok_or(nom::Err::Error(ParseError::from_error_kind(content, ErrorKind::RegexpCapture)))?;

    let matched = matched.get(0).map_or("", |s| s.as_str());
    assert_ne!(matched, "");

    Ok((content.slice(matched.len()..), matched))
}

///
/// Look for the position of file where the input value is located.
///
/// Never forget to give a file name before calling this function.
///
pub fn get_position(file_name: String) -> impl Fn(Span) -> IResult<Span, FilePosition, GSError> {
    move |input| {
        let (input, pos) = position(input)?;
        let pos = FilePosition::from_span(file_name.clone(), &pos);
        Ok((input, pos))
    }
}

///
/// Test for `crate::parsers::utils`.
///
#[cfg(test)]
mod test {
    #[test]
    fn identifier_test1() {
        use crate::parsers::Span;
        use crate::parsers::utils::identifier;

        for name in vec![ "m_WindowWidth", "load3DData!", "__Initialize" ] {
            let val = Span::new(name);
            let val = identifier(val);

            if let Ok((_, parsed)) = val {
                assert_eq!(parsed, name);
            }
            else {
                panic!();
            }
        }
    }

    #[test]
    fn identifier_test2() {
        use crate::parsers::Span;
        use crate::parsers::utils::identifier;

        for name in vec![ "314", "こんにちは!", "😊" ] {
            let val = Span::new(name);
            let val = identifier(val);

            if let Ok((_, _)) = val {
                panic!();
            }
        }
    }
}
