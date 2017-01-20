extern crate rstvnamer;
use rstvnamer::{TvnamerError, TvnamerResult};

use std::path::Path;


enum ActionModes{
    Copy,
    Move,
    Symlink,
}

struct Action{
    mode: ActionModes,
    orig_filepath: String,
    new_name: String,
}

impl Action{
    fn new(orig_filepath: String, new_name: String) -> Action {
        Action{
            mode: ActionModes::Copy, // FIXME
            orig_filepath: orig_filepath,
            new_name: new_name,
        }
    }
    fn perform(&self){
        match self.mode{
            ActionModes::Copy => println!("Copy {} to {}", self.orig_filepath, self.new_name),
            ActionModes::Move => println!("Move!"),
            ActionModes::Symlink => println!("Symlink!"),
        }
    }
}

fn process_one(path: &str) -> Result<(), TvnamerError>{
    let fname = Path::new(path).file_stem()
        .ok_or(TvnamerError::InternalError{
            reason: format!("No file name found for path {}", path)})?
            .to_str().ok_or(TvnamerError::InternalError{
                reason: "Failed to convert to string".into()})?;
    println!("{}", fname);

    let parsed = try!(rstvnamer::parse(fname));
    println!("{:?}", parsed);

    let populated = try!(rstvnamer::populate(parsed));
    println!("{:?}", populated);

    let formatted = try!(rstvnamer::format(populated));
    println!("{} -> {:?}", fname, formatted);
    let act = Action::new(fname.into(), formatted);
    act.perform();
    return Ok(());
}

#[cfg(not(test))]
#[cfg(not(doc))]
fn main(){
    for fname in std::env::args().skip(1) {
        match process_one(&fname) {
            Ok(_) => (),
            Err(e) => println!("Error renaming {}: {}", fname, e),
        };
    }
}
