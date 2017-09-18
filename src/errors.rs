use std::env;
use std::fmt;
use tvdb::TvdbError;

/// All errors which may occur in the library
#[derive(Debug)]
pub enum TvnamerError {
    ParseError{reason: String},
    TvdbError{original: TvdbError},
    InternalError{reason: String},
    FileAlreadyExists{src: String, dest: String, action: String},
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
            TvnamerError::FileAlreadyExists {src: ref src, dest: ref dest, action: ref action} => write!(
                f, "Cannot {} file from '{}' to destination '{}' - destination already exists", action, src, dest),
            TvnamerError::MiscError => write!(f, "Misc error"),
        }
    }
}

impl From<TvdbError> for TvnamerError{
    fn from(err: TvdbError) -> TvnamerError{
        TvnamerError::TvdbError{original: err}
    }
}
