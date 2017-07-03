use std::path::{Path, PathBuf};
use super::{TvnamerError, TvnamerResult};


/// The various operations available to perform with an `Action`
pub enum ActionModes{
    Copy,
    Move,
    Symlink,
}

/// An action which can be performed with a original and new path
pub struct Action<'a>{
    pub mode: ActionModes,
    pub orig_path: &'a Path,
    pub new_name: String,
}

impl<'a> Action<'a>{
    /// Construct new action
    pub fn new(orig_path: &Path, new_name: String, mode: ActionModes) -> Action {
        Action{
            mode: mode,
            orig_path: orig_path,
            new_name: new_name,
        }
    }

    /// Perform given action, returning new path
    pub fn perform(&self) -> TvnamerResult<PathBuf>{
        match self.mode{
            ActionModes::Copy => println!(
                "Copy {:?} to {:?}", self.orig_path, self.new_name),
            ActionModes::Move => println!(
                "Move!"),
            ActionModes::Symlink => println!(
                "Symlink from {:?} to {:?}", self.orig_path, self.new_name),
        }

        Err(TvnamerError::InternalError{reason: format!("not yet implemented!")})
    }
}
