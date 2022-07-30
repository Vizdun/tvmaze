use serde::Deserialize;

use crate::models::{Links, Image, Episode, time::TVMazeTime, Country};

use super::CastMember;

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
pub struct Character {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub image: Option<Image>,
    pub _links: Links,
}