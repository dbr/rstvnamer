use tvdb::TvdbError;


/// All errors which may occur in the library
#[derive(Fail, Debug)]
pub enum TvnamerError {
    #[fail(display = "failed to parse: {}", reason)]
    ParseError { reason: String },
    #[fail(display = "error from TheTVDB: {:?}", original)]
    TvdbError { #[cause] original: TvdbError },
    #[fail(display = "failed to find {}", what)]
    EpisodeNotFound { what: String },
    #[fail(display = "internal library error from tvdb-rs: {:?}", reason)]
    InternalError { reason: String },
    #[fail(display = "cannot {} file from source {} to destination {}: {}", action, src, dest, reason)]
    FileAlreadyExists {
        src: String,
        dest: String,
        action: String,
        reason: String,
    },
    #[fail(display = "misc error")]
    MiscError,
}

/// Shortcut
pub type TvnamerResult<T> = Result<T, TvnamerError>;


impl From<TvdbError> for TvnamerError {
    fn from(err: TvdbError) -> TvnamerError {
        TvnamerError::TvdbError { original: err }
    }
}
