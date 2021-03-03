use std::fmt;
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
    Log
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

impl fmt::Display for FilePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.file_name == "__internal" {
            Ok(())
        }
        else {
            write!(f, "{}:{}:{}", self.file_name, self.line, self.pos)
        }
    }
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

    ///
    /// Return a default value.
    ///
    pub fn compiler() -> Self {
        let file_name = "__internal".to_string();
        Self {
            file_name, line: 0, pos: 0, length: 0
        }
    }
}

impl fmt::Display for ErrorToken  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self.kind {
            ErrorKind::FatalError | ErrorKind::Error => {
                "[ERROR]"
            },
            ErrorKind::Warning => {
                "[WARNING]"
            },
            ErrorKind::Information => {
                "[INFO]"
            },
            ErrorKind::Log => {
                "[LOG]"
            }
        };
        write!(f, "{} {} {}", kind, self.error_message, self.pos)
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
    /// Produce a fatal error token.
    ///
    pub fn fatal_error(error_message: String) -> Self {
        let pos = FilePosition::compiler();
        ErrorToken::new(pos, ErrorKind::FatalError, error_message)
    }

    ///
    /// Produce an error token.
    ///
    pub fn error(pos: FilePosition, error_message: String) -> Self {
        ErrorToken::new(pos, ErrorKind::Error, error_message)
    }

    ///
    /// Produce a warning token.
    ///
    pub fn warn(pos: FilePosition, warn_message: String) -> Self {
        ErrorToken::new(pos, ErrorKind::Warning, warn_message)
    }

    ///
    /// Produce an information token.
    ///
    pub fn info(pos: FilePosition, info_message: String) -> Self {
        ErrorToken::new(pos, ErrorKind::Information, info_message)
    }

    ///
    /// Produce a log token.
    ///
    pub fn log(pos: FilePosition, log_message: String) -> Self {
        ErrorToken::new(pos, ErrorKind::Log, log_message)
    }
}
