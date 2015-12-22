extern crate regex;
extern crate tvdb;

// Loads other files
pub mod utils;
pub mod config;
pub mod parsing;
pub mod populate;

// Reexport public interface
pub use parsing::parse;
pub use populate::populate;
