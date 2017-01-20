extern crate rstvnamer;
use rstvnamer::{TvnamerError, TvnamerResult};


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

fn process_one(fname: &str) -> Result<(), TvnamerError>{
    let parsed = try!(rstvnamer::parse(fname));
    let populated = try!(rstvnamer::populate(parsed));
    let formatted = try!(rstvnamer::format(populated));
    println!("{} -> {:?}", fname, formatted);
    let act = Action::new(fname.into(), formatted);
    act.perform();
    return Ok(());
}

#[cfg(not(test))]
#[cfg(not(doc))]
fn main(){
    //let filenames = vec!["scrubs.s01e22.avi", "the.simpsons.2004.01.12.avi"];
    //for fname in filenames.iter(){
    for fname in std::env::args().skip(1) {
        match process_one(&fname) {
            Ok(_) => (),
            Err(e) => println!("Error renaming {}: {}", fname, e),
        };
    }
}
