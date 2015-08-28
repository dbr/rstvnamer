extern crate rstvnamer;


#[cfg(not(test))]
#[cfg(not(doc))]
fn main(){
    let filenames = vec!["scrubs.s01e22.avi", "scrubs.2004.01.12.avi"];
    for fname in filenames.iter(){
        let parsed = rstvnamer::parse(fname);
        if let Some(p) = parsed {
            println!("Parsed '{}' -> {:?}", fname, p);
            let pop = rstvnamer::populate(p);
            println!("Populated: {:?}", pop);
        } else {
            println!("Failed to parse {}", fname);
        }
    }
}
