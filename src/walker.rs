use crate::prelude::*;
use crate::show::Show;
use std::{fs::read_dir, result::Result as ResultExt};

/// Walk through the directory recursively to parse all the shows matching directory
/// names. For each directory representing a show an episode with the highest season and
/// episode number is found and then the callback is called with the parsed [`Show`].
pub fn walk_dir<F: Fn(Show)>(path: &std::path::Path, callback: &F) -> Result<()> {
    // Collect all the directories and files.
    let mut dirs = vec![];
    let mut files = vec![];

    // Read the directory and filter all non-valid entries.
    for entry in read_dir(path)?.filter_map(ResultExt::ok) {
        // We will save all the files and directories for later.
        match entry.file_type()? {
            t if t.is_dir() => dirs.push(entry.path()),
            t if t.is_file() => files.push(entry.path()),
            _ => (),
        }
    }

    // Try to create a new show from the current directory.
    let mut show = Show::new(path)?;

    // Update the show with the files we found.
    for file in &files {
        show.update(file)?;
    }

    // The show should be valid now. Time for a callback.
    if show.is_valid() {
        callback(show);
    }

    // Recursively walk through sorted directories.
    dirs.sort();
    for dir in dirs {
        walk_dir(&dir, callback)?;
    }

    Ok(())
}
