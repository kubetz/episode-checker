use clap::Parser;
use show::Show;
use std::{fs::canonicalize, path::PathBuf};
use time::Duration;
use walker::walk_dir;

mod api;
mod prelude;
mod show;
mod walker;

/// Tool for checking new TV show episodes.
#[derive(Parser)]
struct Cli {
    /// Directory from which shows will be checked recursively for new episodes.
    dir: Option<String>,
    /// Number of days from today representing the target airdate (default: -1).
    diff: Option<i64>,
}

fn main() {
    // Parse positional command line arguments and provide fallback values.
    let cli = Cli::parse();
    let diff = Duration::days(cli.diff.unwrap_or(-1));
    let dir = cli.dir.unwrap_or(".".into());
    let dir = canonicalize(PathBuf::from(&dir)).expect("Wrong directory!");
    assert!(dir.is_dir(), "The given path is not a directory.");

    // Print the show name and all the episodes that are newer than the latest in the directory.
    let callback = |show: Show| match api::check_api(&show, diff) {
        // If we successfully managed to check the episode, we will print the list of new episodes.
        Ok(episodes) => {
            if !episodes.is_empty() {
                println!("{:30}{:>70}", show.name, show.path.display());
                for e in episodes {
                    println!("\tS{:0>2}E{:0>2}", e.season, e.number);
                }
            }
        }
        Err(e) => eprintln!("{:30}{:>70}\n\tSkipping ({e})", show.name, show.path.display()),
    };

    // Start walking the given directory and call callback for each directory representing a show.
    if let Err(e) = walk_dir(&dir, &callback) {
        eprintln!("Error: {e}");
    }
}
