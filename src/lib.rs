extern crate regex;
extern crate tvdb;


/// Used for air-date of an episode etc
#[derive(Debug)]
pub struct Date {
    year: i32,
    month: i32,
    day: i32,
}


// Loads other files
pub mod utils;
pub mod config;
pub mod parsing;
pub mod populate;

// Reexport public interface
pub use parsing::parse;
pub use populate::populate;
