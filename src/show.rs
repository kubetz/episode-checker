use std::path::Path;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::map_res,
    multi::many_till,
    sequence::{pair, preceded},
    IResult,
};

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
        if let Ok((a, (b, (season, episode)))) = Self::parse_episode(path) {
            dbg!(a, b, season, episode);
            // Always save the newest season and episode number.
            if season > self.season || (season == self.season && episode > self.number) {
                self.season = season;
                self.number = episode;
            }
        }
    }

    /// Checks if the show is valid.
    pub fn is_valid(&self) -> bool {
        self.season > 0 && self.number > 0
    }

    /// Parses the episode from the file name. Result contains rest of the unparsed input,
    /// all the characters before the season and episode number and then the season/number.
    fn parse_episode(input: &Path) -> IResult<&str, (Vec<char>, (u16, u8))> {
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
            // Parse file name as a &str.
        )(input.file_name().unwrap().to_str().unwrap())
    }
}
