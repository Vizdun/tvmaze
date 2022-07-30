use serde::Deserialize;

use super::{
    episodes::Episode,
    people::Person,
    time::{TVMazeDate, TVMazeTime},
    Country, Image, Links, Rating,
};

#[derive(Deserialize, Debug)]
pub struct Show {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub r#type: String,
    pub language: String,
    pub genres: Vec<String>,
    pub status: String,
    pub runtime: Option<usize>,
    #[serde(rename = "averageRuntime")]
    pub average_runtime: Option<usize>,
    pub premiered: Option<TVMazeDate>,
    pub ended: Option<TVMazeDate>,
    #[serde(rename = "officialSite")]
    pub official_site: Option<String>,
    pub schedule: Schedule,
    pub rating: Rating,
    pub weight: usize,
    pub network: Option<Network>,
    #[serde(rename = "webChannel")]
    pub web_channel: Option<Network>,
    #[serde(rename = "dvdCountry")]
    pub dvd_country: Option<Country>,
    pub externals: Externals,
    pub image: Option<Image>,
    pub summary: Option<String>,
    pub updated: u64,
    pub _links: Links,
    pub _embedded: Option<Embedded>,
}

#[derive(Deserialize, Debug)]
pub struct AlternateList {
    pub id: usize,
    pub url: String,
    pub dvd_release: bool,
    pub verbatim_order: bool,
    pub country_premiere: bool,
    pub streaming_premiere: bool,
    pub broadcast_premiere: bool,
    pub language_premiere: bool,
    pub language: Option<String>,
    pub network: Option<Network>,
    #[serde(rename = "webChannel")]
    pub web_channel: Option<Network>,
    pub _links: Links,
}

#[derive(Deserialize, Debug)]
pub struct Season {
    pub id: usize,
    pub url: String,
    pub number: usize,
    pub name: String,
    #[serde(rename = "episodeOrder")]
    pub episode_order: Option<usize>,
    #[serde(rename = "premiereDate")]
    pub premiere_date: Option<TVMazeDate>,
    #[serde(rename = "endDate")]
    pub end_date: Option<TVMazeDate>,
    pub network: Option<Network>,
    #[serde(rename = "webChannel")]
    pub web_channel: Option<Network>,
    pub image: Option<Image>,
    pub summary: Option<String>,
    pub _links: Links,
}

#[derive(Deserialize, Debug)]
pub struct Aka {
    pub name: String,
    pub country: Option<Country>,
}

#[derive(Deserialize, Debug)]
pub struct ShowImage {
    pub id: usize,
    pub r#type: String,
    pub main: bool,
    pub resolutions: Resolutions,
}

#[derive(Deserialize, Debug)]
pub struct Resolutions {
    pub original: Resolution,
    pub medium: Option<Resolution>,
}

#[derive(Deserialize, Debug)]
pub struct Resolution {
    pub url: String,
    pub width: usize,
    pub height: usize,
}

#[derive(Deserialize, Debug)]
pub struct Network {
    pub id: usize,
    pub name: String,
    pub country: Option<Country>,
    #[serde(rename = "officialSite")]
    pub official_site: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Externals {
    pub tvrage: Option<usize>,
    pub thetvdb: Option<usize>,
    pub imdb: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Schedule {
    pub time: TVMazeTime,
    pub days: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Embedded {
    pub episodes: Option<Vec<Episode>>,
    pub cast: Option<Vec<CastMember>>,
    pub nextepisode: Option<Episode>,
}

#[derive(Deserialize, Debug)]
pub struct CrewMember {
    pub r#type: String,
    pub person: Person,
}

#[derive(Deserialize, Debug)]
pub struct CastMember {
    pub person: Person,
    pub character: Character,
    #[serde(rename = "self")]
    pub _self: bool,
    pub voice: bool,
}

#[derive(Deserialize, Debug)]
pub struct Character {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub image: Option<Image>,
    pub _links: Links,
}
