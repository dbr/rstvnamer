/// Errors in contacting TheTVDB
#[derive(Debug)]
pub enum TvdbError {
    SeriesNotFound,
    CommunicationError{reason: String},
}
