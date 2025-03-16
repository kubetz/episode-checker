use crate::prelude::*;
use std::path::{Path, PathBuf};

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::map_res,
    multi::many_till,
    sequence::{pair, preceded},
    Parser,
};

/// Show represents ... wait for it ... a TV show 🤯.
/// Season and number types must be in sync with [`crate::checker::Episode`].
#[derive(Debug)]
pub struct Show {
    pub name: String,
    pub path: PathBuf,
    pub season: u16,
    pub number: u8,
}

impl Show {
    /// Creates a new show from the path of the directory representing the name of the show.
    /// Initial season and episode numbers are set to 0 as we will parse them from files later.
    pub fn new(path: PathBuf) -> Result<Self> {
        Ok(Self {
            name: Self::path_to_file_name(&path)?,
            path,
            season: 0,
            number: 0,
        })
    }

    /// Updates the show with the season and episode based on the file path.
    pub fn update(&mut self, path: &Path) -> Result<()> {
        if let Ok((season, episode)) = Self::parse_episode(&Self::path_to_file_name(path)?) {
            // Always save the newest season and episode number.
            if season > self.season || (season == self.season && episode > self.number) {
                self.season = season;
                self.number = episode;
            }
        }
        Ok(())
    }

    /// Checks if the show is valid.
    pub fn is_valid(&self) -> bool {
        self.season > 0 && self.number > 0
    }

    /// Parses the episode from the file name. Result contains rest of the unparsed input,
    /// all the characters before the season and episode number and then the season/number.
    fn parse_episode(name: &str) -> Result<(u16, u8)> {
        many_till(
            // Match any character until the second parser matches.
            anychar,
            // We are matching a pair that get parsed into u16 and u8.
            pair(
                // First is a season that is preceded by "S".
                map_res(preceded(tag("S"), digit1), str::parse::<u16>),
                // Right after it there should be an episode number preceded by "E".
                map_res(preceded(tag("E"), digit1), str::parse::<u8>),
            ),
        )
        // Parse file name as a &str.
        .parse(name)
        // Remove useless parts of the result.
        .map(|(_, (_, (season, episode)))| (season, episode))
        // Convert nom error to our error.
        .map_err(Error::Parse)
    }

    /// Returns the file name from the path.
    fn path_to_file_name(path: &Path) -> Result<String> {
        match path.file_name() {
            Some(name) => name
                .to_str()
                .map(String::from)
                .ok_or(Error::InvalidFile(path.to_path_buf())),
            None => Err(Error::InvalidFile(path.to_path_buf())),
        }
    }
}
