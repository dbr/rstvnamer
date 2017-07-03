use std::env;
use std::fmt;
use tvdb::TvdbError;

/// All errors which may occur in the library
#[derive(Debug)]
pub enum TvnamerError {
    ParseError{reason: String},
    TvdbError{original: TvdbError},
    InternalError{reason: String},
    MiscError,
}

/// Shortcut
pub type TvnamerResult<T> = Result<T, TvnamerError>;

// Formatting for error
impl fmt::Display for TvnamerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TvnamerError::ParseError{reason: ref e} => write!(f, "{}", e),
            TvnamerError::TvdbError{original: ref e} => write!(f, "{}", e),
            TvnamerError::InternalError{reason: ref e} => write!(f, "Internal error: {}", e),
            TvnamerError::MiscError => write!(f, "Misc error"),
        }
    }
}

impl From<TvdbError> for TvnamerError{
    fn from(err: TvdbError) -> TvnamerError{
        TvnamerError::TvdbError{original: err}
    }
}
