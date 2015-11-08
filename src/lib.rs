extern crate regex;
extern crate hyper;


/// Used for air-date of an episode etc
#[derive(Debug)]
pub struct Date {
    year: i32,
    month: i32,
    day: i32,
}


// Loads other files
mod utils;
mod config;
mod parsing;
mod populate;
pub mod tvdb;

// Reexport public interface
pub use parsing::parse;
pub use populate::populate;
