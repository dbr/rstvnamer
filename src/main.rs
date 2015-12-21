extern crate rstvnamer;

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

#[cfg(not(test))]
#[cfg(not(doc))]
fn main(){
    let filenames = vec!["scrubs.s01e22.avi", "the.simpsons.2004.01.12.avi"];
    for fname in filenames.iter(){
        let parsed = rstvnamer::parse(fname).unwrap();
        let populated = rstvnamer::populate::populate(parsed);
        println!("{:?}", populated);
    }
}
