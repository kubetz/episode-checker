use std::path::Path;

use clap::Parser;
use show::Show;
use walker::walker;

#[macro_use]
extern crate lazy_static;

mod api;
mod show;
mod walker;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    dir: String,

    #[arg(long, default_value_t = -1)]
    diff: i64,
}

// TODO: Write a README.md

fn main() {
    let cli = Cli::parse();

    // Print the show name and all the episodes that are newer than the latest in the directory.
    let callback = |show: Show| {
        if let Some(episodes) = api::check_api(&show, cli.diff) {
            println!("{}", show.name);
            episodes
                .iter()
                .for_each(|e| println!("\tS{:0>2}E{:0>2}", e.season, e.number));
        }
    };

    // Start walking the given directory and call callback for each directory representing a show.
    walker(Path::new(&cli.dir), &callback).expect("Problem processing directories");
}
