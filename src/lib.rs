extern crate regex;
extern crate tvdb;
extern crate symlink;

// Loads other files
pub mod utils;
mod config;
mod parsing;
mod populate;
mod format;
mod errors;
mod actions;

// Reexport public interface
pub use errors::TvnamerError;
pub use errors::TvnamerResult;

pub use actions::Action;
pub use actions::ActionModes;

pub use parsing::ParsedFile;
pub use populate::PopulatedFile;

pub use format::format;
pub use parsing::parse;
pub use populate::populate;
