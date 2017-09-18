extern crate rstvnamer;
extern crate clap;
use rstvnamer::TvnamerResult;

use std::path::{Path, PathBuf};
use clap::{Arg, App};

fn process_one(path: &Path) -> TvnamerResult<PathBuf> {
    let parsed = rstvnamer::parse(path)?;
    println!("{:?}", parsed);

    let populated = rstvnamer::populate(&parsed)?;
    println!("{:?}", populated);

    let formatted = rstvnamer::format(&populated, &parsed, &path)?;
    println!("{:?} formats into {:?}", populated, formatted);

    let act = rstvnamer::Action::new(&path, formatted, rstvnamer::ActionModes::Symlink);
    return act.perform();
}

#[cfg(not(test))]
#[cfg(not(doc))]
fn main() {
    let matches = App::new("tvnamer")
        .about("Automatic TV episode namer")
        .arg(
            Arg::with_name("files")
                .required(true)
                .takes_value(true)
                .multiple(true)
                .help("files to rename"),
        )
        .get_matches();

    let args: Vec<&str> = matches.values_of("files").unwrap().collect();

    for fname in args {
        println!("# Processing: {}", fname);
        match process_one(&Path::new(&fname)) {
            Ok(_) => (),
            Err(e) => println!("Error renaming {}: {}", fname, e),
        };
    }
}
