extern crate rstvnamer;
use rstvnamer::{TvnamerError, TvnamerResult};

use std::path::{Path, PathBuf};

fn process_one(path: &Path) -> TvnamerResult<PathBuf>{
    let parsed = try!(rstvnamer::parse(path));
    println!("{:?}", parsed);

    let populated = try!(rstvnamer::populate(&parsed));
    println!("{:?}", populated);

    let formatted = try!(rstvnamer::format(&populated, &parsed, &path));
    println!("{:?} formats into {:?}", populated, formatted);

    let act = rstvnamer::Action::new(&path, formatted, rstvnamer::ActionModes::Symlink);
    return act.perform();
}

#[cfg(not(test))]
#[cfg(not(doc))]
fn main(){
    for fname in std::env::args().skip(1) {
        match process_one(&Path::new(&fname)) {
            Ok(_) => (),
            Err(e) => println!("Error renaming {}: {}", fname, e),
        };
    }
}
