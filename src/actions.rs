use std::path::Path;

/// Different operations to perform with the new name
pub enum ActionModes{
    Copy,
    Move,
    Symlink,
}

pub struct Action<'a>{
    pub mode: ActionModes,
    pub orig_path: &'a Path,
    pub new_name: String,
}

impl<'a> Action<'a>{
    pub fn new(orig_path: &Path, new_name: String, mode: ActionModes) -> Action {
        Action{
            mode: mode,
            orig_path: orig_path,
            new_name: new_name,
        }
    }
    pub fn perform(&self){
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
