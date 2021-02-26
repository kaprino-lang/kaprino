use nom_locate::LocatedSpan;

///
/// `ErrorKind` stores some constant values with which you can represent a kind of error.
///
/// It can be used with `ErrorToken`.
///
#[derive(Debug,PartialEq)]
pub enum ErrorKind {
    FatalError,
    Error,
    Warning,
    Information,
    Log,
    InternalLog
}

///
/// `FilePosition` represents the position of the file.
///
/// It can be used with `ErrorToken`.
///
#[derive(Debug,PartialEq,Clone)]
pub struct FilePosition {
    file_name: String,
    line: u64,
    pos: u64,
    length: u64
}

///
/// `ErrorToken` is a token which stores information about the error occurred.
///
#[derive(Debug,PartialEq)]
pub struct ErrorToken {
    kind: ErrorKind,
    error_message: String,
    pos: FilePosition
}

impl FilePosition {
    ///
    /// Create a `FilePosition` instance.
    ///
    pub fn new(file_name: String, line: u64, pos: u64, length: u64) -> Self {
        Self {
            file_name, line, pos, length
        }
    }

    ///
    /// Convert from `nom_locate::LocatedSpan`.
    ///
    pub fn from_span<'a>(file_name: String, span: &LocatedSpan<&'a str>) -> Self {
        let line: u64 = span.location_line().into();
        let pos = span.get_column() as u64;
        let length = span.fragment().len() as u64;
        Self {
            file_name, line, pos, length
        }
    }
}

impl ErrorToken {
    ///
    /// Create a `ErrorToken` instance.
    ///
    /// You need specify a kind of notifications.
    ///
    pub fn new(pos: FilePosition, kind: ErrorKind, error_message: String) -> Self {
        Self {
            pos, kind, error_message
        }
    }

    ///
    /// Produce an error token.
    ///
    pub fn error(pos: FilePosition, error_message: String) -> Self {
        ErrorToken::new(pos, ErrorKind::Error, error_message)
    }
}
