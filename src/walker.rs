use std::{fs::read_dir, path::PathBuf};

use crate::prelude::*;
use crate::show::Show;

/// Walk through the directory recursively to parse all the shows matching directory
/// names. For each directory representing a show an episode with highest season and
/// episode number is found and then the callback is called with the parsed [`Show`].
pub fn walker<F: Fn(Show)>(path: PathBuf, callback: &F) -> Result<()> {
    // Collect all the directories and files.
    let mut dirs = vec![];
    let mut files = vec![];

    // Read the directory and filter all non-valid entries.
    for entry in read_dir(path.clone())?.filter_map(|e| e.ok()) {
        // We will save all the files and directories for for later.
        match entry.file_type()? {
            t if t.is_dir() => dirs.push(entry.path()),
            t if t.is_file() => files.push(entry.path()),
            _ => (),
        }
    }

    // Try to create a new show from the current directory.
    let mut show = Show::new(path)?;

    // Update the show with the files we found.
    for file in files {
        show.update(file)?;
    }

    // The show should be valid now. Time for a callback.
    if show.is_valid() {
        callback(show);
    }

    // Recursively walk through all the directories.
    for dir in dirs {
        walker(dir, callback)?;
    }

    Ok(())
}
