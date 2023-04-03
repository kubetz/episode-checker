use std::path::Path;

use regex::Regex;

lazy_static! {
    // Prepare the season/episode regex once and use it for all the files.
    static ref SEASON_REGEXP: Regex = Regex::new(r"S(\d+)E(\d+)").unwrap();
}

/// Show represents ... wait for it ... a TV show 🤯.
/// Season and number types must be in sync with [`crate::checker::Episode`].
#[derive(Debug)]
pub struct Show<'a> {
    pub name: &'a str,
    pub season: u16,
    pub number: u8,
}

impl<'a> Show<'a> {
    /// Creates a new show from the path of the directory representing the name of the show.
    /// Initial season and episode numbers are set to 0 as we will parse them from files later.
    pub fn new(path: &'a Path) -> Option<Self> {
        path.file_name().map(|name| Self {
            name: name.to_str().unwrap(),
            season: 0,
            number: 0,
        })
    }

    /// Updates the show with the season and episode based on the file path.
    pub fn update(&mut self, path: &Path) {
        if let Some(caps) = SEASON_REGEXP.captures(path.to_str().unwrap()) {
            let season: u16 = caps.get(1).unwrap().as_str().parse().unwrap();
            let episode: u8 = caps.get(2).unwrap().as_str().parse().unwrap();

            // Always save the newest season and episode number.
            if season > self.season {
                self.season = season;
                self.number = episode;
            } else if season == self.season && episode > self.number {
                self.number = episode;
            }
        }
    }

    /// Checks if the show is valid.
    pub fn is_valid(&self) -> bool {
        self.season > 0 && self.number > 0
    }
}
