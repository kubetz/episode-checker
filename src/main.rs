use clap::Parser;
use prelude::*;
use show::Show;
use std::{fs::canonicalize, path::PathBuf, process};
use time::Duration;
use walker::walker;

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

fn main() -> Result<()> {
    // Parse positional command line arguments and provide fallback values.
    let cli = Cli::parse();
    let diff = Duration::days(cli.diff.unwrap_or(-1));
    let dir = cli.dir.unwrap_or(".".into());
    let dir = canonicalize(PathBuf::from(&dir)).expect("Wrong directory!");
    assert!(dir.is_dir(), "The given path is not a directory.");

    // Print the show name and all the episodes that are newer than the latest in the directory.
    let callback = |show: Show| match api::check_api(&show, diff) {
        // If we successfully managed to check the episode, we will print the show
        // name and if list of new episodes or inform the user that there are none.
        Ok(episodes) => {
            println!("{}", show.name);
            match episodes.len() {
                0 => println!("\tnone"),
                _ => episodes
                    .iter()
                    .for_each(|e| println!("\tS{:0>2}E{:0>2}", e.season, e.number)),
            }
        }
        Err(e) => match e {
            // If we cannot load the show data, it likely means it is not a directory of the show.
            prelude::Error::CannotLoad() | prelude::Error::NotFound(_, _) => {
                eprintln!("{}\n\tskipping ({e})", show.name);
            }
            // Rest of the errors is ... a show stopper 😏.
            _ => {
                eprintln!("{}\n\texiting ({e})", show.name);
                process::exit(1);
            }
        },
    };

    // Start walking the given directory and call callback for each directory representing a show.
    walker(dir, &callback)?;

    Ok(())
}
