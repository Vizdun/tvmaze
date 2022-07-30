use serde::Deserialize;

use super::{
    people::Person,
    time::{TVMazeDate},
    Country, Image, Links, Rating,
};

mod children;
pub use children::*;
mod enums;
pub use enums::*;

#[derive(Deserialize, Debug)]
pub struct Show {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub r#type: ShowType,
    pub language: Language,
    pub genres: Vec<Genre>,
    pub status: ShowStatus,
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