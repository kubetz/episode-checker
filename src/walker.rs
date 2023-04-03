use std::{fs, io, path::Path};

use crate::show::Show;

pub fn walker<F>(path: &Path, callback: &F) -> io::Result<()>
where
    F: Fn(Show),
{
    assert!(path.is_dir());

    // First collect all the directories and files.
    let mut dirs = vec![];
    let mut files = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        match entry.file_type()? {
            t if t.is_dir() => dirs.push(entry.path()),
            t if t.is_file() => files.push(entry.path()),
            _ => (),
        }
    }

    // Try to create a new show from the current directory.
    // If it is possible, we will start processing files to update the show.
    if let Some(mut show) = Show::new(path) {
        // Update the show with the files we found.
        for file in files {
            show.update(&file);
        }

        // The show should be valid now. Time for a callback.
        if show.is_valid() {
            callback(show);
        }
    }

    // Recursively walk through all the directories.
    for dir in dirs {
        walker(&dir, callback)?;
    }

    Ok(())
}
