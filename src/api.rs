use serde::{Deserialize, Deserializer};
use time::{macros::format_description, Date, Duration, OffsetDateTime};

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

/// Single episode. Returned from [`check_api`] if it's newer than the given one.
#[derive(Deserialize)]
pub struct Episode {
    /// Air date of the episode. Parsed to [`time::Date`] with a custom deserializer.
    #[serde(deserialize_with = "deserialize_date", rename = "airdate")]
    pub date: Date,
    /// Season of the episode. Some shows use year here.
    /// That is the reason we are using bigger integer type.
    pub season: u16,
    /// Episode number.
    pub number: u8,
}

/// Deserializes the date from the string in the format "YYYY-MM-DD".
fn deserialize_date<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let str: String = Deserialize::deserialize(deserializer)?;
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(&str, &format).map_err(serde::de::Error::custom)
}

/// Uses TVMaze API to check if there are any new episodes for the given show.
/// Returns None if no newer episodes were found. Episode number and season
/// must be non-zero. Exits gracefully if the show or the specific episode
/// cannot be found as that could be caused by a random directory. However
/// issues with parsing will panic as that means incompatible API.
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

    // Find the airdate of the given show. If no episode could be found, we exit gracefully.
    let cur_date = match json_iter.find(|e| e.season == show.season && e.number == show.number) {
        Some(e) => e.date,
        None => return None,
    };

    // Get target time to make sure we won't include episodes that will air after that.
    let target_date = OffsetDateTime::now_utc().date() + duration_diff;

    // Collect all episodes that are newer than the given one.
    let episodes: Vec<Episode> = json_iter
        .filter(|s| s.date > cur_date && s.date < target_date)
        .collect();

    match episodes.is_empty() {
        true => None,
        false => Some(episodes),
    }
}
