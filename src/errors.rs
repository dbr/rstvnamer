use std::env;
use std::fmt;
use tvdb::TvdbError;

/// All errors which may occur in the library
#[derive(Debug)]
pub enum TvnamerError {
    ParseError { reason: String },
    TvdbError { original: TvdbError },
    InternalError { reason: String },
    FileAlreadyExists {
        src: String,
        dest: String,
        action: String,
        reason: String,
    },
    MiscError,
}

/// Shortcut
pub type TvnamerResult<T> = Result<T, TvnamerError>;

// Formatting for error
impl fmt::Display for TvnamerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TvnamerError::ParseError { ref reason } => write!(f, "{}", reason),
            TvnamerError::TvdbError { ref original } => write!(f, "{}", original),
            TvnamerError::InternalError { ref reason } => write!(f, "Internal error: {}", reason),
            TvnamerError::FileAlreadyExists {
                ref src,
                ref dest,
                ref action,
                ref reason,
            } => {
                write!(
                    f,
                    "Cannot {} file from '{}' to destination '{}': {}",
                    action,
                    src,
                    dest,
                    reason,
                )
            }
            TvnamerError::MiscError => write!(f, "Misc error"),
        }
    }
}

impl From<TvdbError> for TvnamerError {
    fn from(err: TvdbError) -> TvnamerError {
        TvnamerError::TvdbError { original: err }
    }
}
