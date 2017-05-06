extern crate rstvnamer;
use rstvnamer::{TvnamerError, TvnamerResult};

use std::path::Path;


enum ActionModes{
    Copy,
    Move,
    Symlink,
}

struct Action<'a>{
    mode: ActionModes,
    orig_path: &'a Path,
    new_name: String,
}

impl<'a> Action<'a>{
    fn new(orig_path: &Path, new_name: String, mode: ActionModes) -> Action {
        Action{
            mode: mode,
            orig_path: orig_path,
            new_name: new_name,
        }
    }
    fn perform(&self){
        match self.mode{
            ActionModes::Copy => println!(
                "Copy {:?} to {:?}", self.orig_path, self.new_name),
            ActionModes::Move => println!(
                "Move!"),
            ActionModes::Symlink => println!(
                "Symlink from {:?} to {:?}", self.orig_path, self.new_name),
        }
    }
}

fn process_one(path: &Path) -> Result<(), TvnamerError>{
    let parsed = try!(rstvnamer::parse(path));
    println!("{:?}", parsed);

    let populated = try!(rstvnamer::populate(parsed));
    println!("{:?}", populated);

    let formatted = try!(rstvnamer::format(&populated));
    println!("{:?} formats into {:?}", populated, formatted);

    let act = Action::new(&path, formatted, ActionModes::Symlink);
    act.perform();
    return Ok(());
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
