use serde::{Deserialize, Deserializer};
use time::{macros::format_description, Date, Duration, OffsetDateTime};

use crate::prelude::*;
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
    /// Air date of the episode. Parsed to [`Date`] with a custom deserializer.
    #[serde(deserialize_with = "deserialize_date", rename = "airdate")]
    pub date: Date,
    /// Season of the episode. Some shows use year here.
    /// That is the reason we are using bigger integer type.
    pub season: u16,
    /// Episode number.
    pub number: u8,
}

/// Deserializes the date from the string in the format "YYYY-MM-DD".
fn deserialize_date<'de, D>(deserializer: D) -> std::result::Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let str: String = Deserialize::deserialize(deserializer)?;
    // To simplify things empty air date is transformed to an irrelevant date.
    if str.is_empty() {
        return Ok(Date::MIN);
    }
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(&str, &format).map_err(serde::de::Error::custom)
}

/// Uses TVMaze API to check if there are any new episodes for
/// the given show. Episode number and season must be non-zero.
pub fn check_api(show: &Show, duration_diff: Duration) -> Result<Vec<Episode>> {
    // Create an url for the API endpoint that contains encoded show name.
    // Ask or all episode data right away to minimize the number of requests.
    let url = format!(
        "https://api.tvmaze.com/singlesearch/shows?q={}&embed=episodes",
        urlencoding::encode(show.name.as_str())
    );

    // Deserialize all relevant json data and get the iterator.
    let show_json: MazeShow = match ureq::get(&url).call() {
        Ok(mut res) => res.body_mut().read_json().map_err(Error::WrongJSON)?,
        Err(_) => return Err(Error::CannotLoad()),
    };
    let json_iter = &mut show_json._embedded.episodes.into_iter();

    // Find the air date of the given show.
    let cur_date = json_iter
        .find(|e| (e.season, e.number) == (show.season, show.number))
        .ok_or(Error::NotFound(show.season, show.number))?
        .date;

    // Get target time to make sure we won't include episodes that will air after that.
    let target_date = OffsetDateTime::now_utc().date() + duration_diff;

    // Collect all episodes that were already released and are following current episode.
    let episodes: Vec<Episode> = json_iter
        .filter(|s| {
            s.date >= cur_date
                && s.date < target_date
                && (s.season, s.number) != (show.season, show.number)
        })
        .collect();

    Ok(episodes)
}
