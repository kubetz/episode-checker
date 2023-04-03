use serde::Deserialize;
use time::macros::format_description;
use time::{Date, Duration, OffsetDateTime};

use crate::show::Show;

/// Internal struct to deserialize the json data.
#[derive(Deserialize)]
struct MazeShow {
    _embedded: MazeEmbedded,
}

/// Internal struct to deserialize the embedded episodes.
#[derive(Deserialize)]
struct MazeEmbedded {
    episodes: Vec<Episode>,
}

/// Single retrieved episode.
#[derive(Deserialize)]
pub struct Episode {
    pub airdate: String,
    /// Some shows on TVMaze have weird season numbering (e.g. using year there).
    pub season: u16,
    pub number: u8,
}

/// Built-in conversion from String to Date. We don't use Option and instead panic as
/// it should be called on Strings that are expected to contain date. If that keeps
/// failing it means the API got changed and the code needs to be updated.
trait AsDate {
    fn as_date(&self) -> Date;
}

impl AsDate for String {
    fn as_date(&self) -> Date {
        let format = format_description!("[year]-[month]-[day]");
        Date::parse(self, format).expect("Wrong airdate format")
    }
}

/// Uses TVMaze API to check if there are any new episodes for the given show.
/// Returns None if no newer episodes were found. Show name must be present and
/// available on TVMaze. Episode number and season must be non-zero.
/// If exit gracefully if the show or the specific episode cannot be found as
/// that could be caused by a random directory. However issues with parsing will
/// panic as that shouldn't normally happen and should be addressed.
pub fn check_api(show: &Show, duration_diff: Duration) -> Option<Vec<Episode>> {
    let url = format!(
        "https://api.tvmaze.com/singlesearch/shows?q={}&embed=episodes",
        urlencoding::encode(show.name)
    );

    // Deserialize all relevant json data and get the iterator.
    let show_json: MazeShow = match ureq::get(&url).call() {
        Ok(res) => res.into_json().expect("Wrong JSON data"),
        Err(_) => return None,
    };
    let json_iter = &mut show_json._embedded.episodes.into_iter();

    // Find the airdate of the given show and parse it. If no episode could be found, we exit gracefully.
    let cur_date = match json_iter.find(|e| e.season == show.season && e.number == show.number) {
        Some(e) => e.airdate.as_date(),
        None => return None,
    };

    // Get target time to make sure we won't be listing episodes that air after that.
    let target_date = OffsetDateTime::now_utc().date() + duration_diff;

    // Collect all shows that are newer than the given one and do it in a inefficient, but stylish way 🤐.
    let episodes: Vec<Episode> = json_iter
        .filter(|s| s.airdate.as_date() > cur_date && s.airdate.as_date() < target_date)
        .collect();

    match episodes.is_empty() {
        true => None,
        false => Some(episodes),
    }
}
