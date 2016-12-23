extern crate rstvnamer;
use std::fmt;

/*
struct ConsoleInput;
impl<'a> ConsoleInput{
    pub fn new() -> ConsoleInput {
        ConsoleInput
    }
}


impl<'a> SeriesSelector for &'a ConsoleInput{
    fn select(self, results: &Vec<SeriesSearchResult>) -> Option<SeriesSearchResult> {
        return Some(results[0].clone());
    }
}
*/

#[derive(Debug)]
pub enum TvnamerError {
    ParseError{reason: String},
    TvdbError{original: rstvnamer::TvdbError},
    MiscError,
}

/// Shortcut
pub type TvnamerResult<T> = Result<T, TvnamerError>;

// Formatting for error
impl fmt::Display for TvnamerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TvnamerError::ParseError{reason: ref e} => write!(f, "Internal error: {}", e),
            TvnamerError::TvdbError{original: ref e} => write!(f, "{}", e),
            TvnamerError::MiscError => write!(f, "Misc error"),
        }
    }
}

impl From<rstvnamer::TvdbError> for TvnamerError{
    fn from(err: rstvnamer::TvdbError) -> TvnamerError{
        TvnamerError::TvdbError{original: err}
    }
}

fn rename_one(fname: &str) -> Result<(), TvnamerError>{
    let parsed = try!(rstvnamer::parse(fname).ok_or(TvnamerError::ParseError{reason: "Failed to parse".into()}));
    let populated = try!(rstvnamer::populate(parsed));
    let formatted = try!(rstvnamer::format(populated));
    println!("{} -> {:?}", fname, formatted);
    return Ok(());
}

#[cfg(not(test))]
#[cfg(not(doc))]
fn main(){
    let filenames = vec!["scrubs.s01e22.avi", "the.simpsons.2004.01.12.avi"];
    for fname in filenames.iter(){
        match rename_one(fname) {
            Ok(_) => (),
            Err(e) => println!("Error renaming {}: {}", fname, e),
        };
    }
}
