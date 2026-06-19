use core::fmt;

/// A generic error type for the rvlibs ecosystem.
///
/// Individual crates may define their own error enums; this type is used
/// at the boundaries between crates.
#[derive(Debug)]
pub enum Error {
    NotFound(String),
    InvalidInput(String),
    Internal(String),
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound(msg) => write!(f, "not found: {msg}"),
            Error::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            Error::Internal(msg) => write!(f, "internal error: {msg}"),
            Error::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

/// Convenience alias for `Result<T, rvlibs::Error>`.
pub type Result<T> = std::result::Result<T, Error>;
