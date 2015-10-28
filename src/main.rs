extern crate rstvnamer;


#[cfg(not(test))]
#[cfg(not(doc))]
fn main(){
    let filenames = vec!["scrubs.s01e22.avi", "the.simpsons.2004.01.12.avi"];
    for fname in filenames.iter(){
        let parsed = rstvnamer::parse(fname);
        if let Some(p) = parsed {
            println!("Parsed '{}' -> {:?}", fname, p);
            let pop = rstvnamer::populate(p);
            match pop {
                Ok(p) => println!("Success: {:?}", p),
                Err(e) => println!("Error! {:?}", e),
            }
        } else {
            println!("Failed to parse {}", fname);
        }
    }
}
