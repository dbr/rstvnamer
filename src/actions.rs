use std::path::{Path, PathBuf};
use symlink;
use super::{TvnamerError, TvnamerResult};


/// The various operations available to perform with an `Action`
pub enum ActionModes {
    Copy,
    Move,
    Symlink,
}

/// An action which can be performed with a original and new path
pub struct Action<'a> {
    pub mode: ActionModes,
    pub orig_path: &'a Path,
    pub new_name: String,
}

impl<'a> Action<'a> {
    /// Construct new action
    pub fn new(orig_path: &Path, new_name: String, mode: ActionModes) -> Action {
        Action {
            mode,
            orig_path,
            new_name,
        }
    }

    /// Perform given action, returning new path
    pub fn perform(&self) -> TvnamerResult<PathBuf> {
        match self.mode {
            ActionModes::Copy => copy_file(self.orig_path, &self.new_name),
            ActionModes::Move => move_file(self.orig_path, &self.new_name),
            ActionModes::Symlink => symlink_file(self.orig_path, &self.new_name),
        }
    }
}


fn symlink_file(old: &Path, new: &String) -> TvnamerResult<(PathBuf)> {
    debug!("Symlinking {:?} with new name {:?}", old, new);
    let parent = old.parent().unwrap();
    let new_filepath = parent.join(new);

    symlink::symlink_file(&old, &new_filepath).or_else(|e| {
        Err(TvnamerError::FileAlreadyExists {
            src: old.to_string_lossy().into(),
            dest: new_filepath.to_string_lossy().into(),
            action: "symlink".into(),
            reason: format!("{}", e),
        })
    })?;

    Ok(new_filepath)
}

fn move_file(old: &Path, new: &String) -> TvnamerResult<(PathBuf)> {
    println!("moving");
    Err(TvnamerError::InternalError {
        reason: "Not not implemented".into(),
    })
}

fn copy_file(old: &Path, new: &String) -> TvnamerResult<PathBuf> {
    println!("copying");
    Err(TvnamerError::InternalError {
        reason: "Not not implemented".into(),
    })
}
