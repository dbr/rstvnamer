extern crate rstvnamer;

use rstvnamer::tvdb::{SeriesSelector, SeriesSearchResult};


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


#[cfg(not(test))]
#[cfg(not(doc))]
fn main(){
    let ui = ConsoleInput::new();

    let filenames = vec!["scrubs.s01e22.avi", "the.simpsons.2004.01.12.avi"];
    for fname in filenames.iter(){
        let parsed = rstvnamer::parse(fname);
        if let Some(p) = parsed {
            println!("Parsed '{}' -> {:?}", fname, p);
            let pop = rstvnamer::populate(p, &ui);
            match pop {
                Ok(p) => println!("Success: {:?}", p),
                Err(e) => println!("Error! {:?}", e),
            }
        } else {
            println!("Failed to parse {}", fname);
        }
    }
}
