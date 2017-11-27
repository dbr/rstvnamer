extern crate rstvnamer;
extern crate clap;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate failure;

use rstvnamer::TvnamerResult;

use std::path::{Path, PathBuf};
use clap::{Arg, App};


fn process_one(path: &Path) -> TvnamerResult<PathBuf> {
    let parsed = rstvnamer::parse(path)?;
    debug!("Parsed {:?} into {:?}", path, parsed);

    let populated = rstvnamer::populate(&parsed)?;
    debug!("Populated: {:?}", populated);

    let formatted = rstvnamer::format(&populated, &parsed, path)?;
    debug!("Formatted {:?}", formatted);

    let act = rstvnamer::Action::new(path, formatted, rstvnamer::ActionModes::Symlink);

    return act.perform();
}


#[cfg(not(test))]
#[cfg(not(doc))]
fn main() {
    env_logger::init().expect("Failed to setup env_logger");

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
        match process_one(Path::new(&fname)) {
            Ok(_) => (),
            Err(e) => println!("Error with {}: {}", fname, e),
        };
    }

}
