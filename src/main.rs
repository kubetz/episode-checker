use std::path::Path;

use clap::Parser;
use show::Show;
use time::Duration;
use walker::walker;

mod api;
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
    let dir = Path::new(&dir);
    assert!(dir.is_dir(), "The given path is not a directory.");

    // Print the show name and all the episodes that are newer than the latest in the directory.
    let callback = |show: Show| {
        if let Some(episodes) = api::check_api(&show, diff) {
            println!("{}", show.name);
            episodes
                .iter()
                .for_each(|e| println!("\tS{:0>2}E{:0>2}", e.season, e.number));
        }
    };

    // Start walking the given directory and call callback for each directory representing a show.
    walker(Path::new(&dir), &callback).expect("Problem processing directories");
}
