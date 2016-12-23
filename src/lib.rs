extern crate regex;
extern crate tvdb;

// Loads other files
pub mod utils;
pub mod config;
pub mod parsing;
pub mod populate;
pub mod format;

// Reexport public interface
pub use tvdb::TvdbResult;
pub use tvdb::TvdbError;
pub use parsing::parse;
pub use populate::populate;
pub use format::format;
