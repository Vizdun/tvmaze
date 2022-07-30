use serde::Deserialize;

use super::{time::TVMazeDate, Country, Image, Links};

#[derive(Deserialize, Debug)]
pub struct Person {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub country: Option<Country>,
    pub birthday: Option<TVMazeDate>,
    pub deathday: Option<TVMazeDate>,
    pub gender: Option<String>,
    pub image: Option<Image>,
    pub updated: usize,
    pub _links: Links,
}

#[derive(Deserialize, Debug)]
pub struct CrewCredit {
    pub r#type: String,
    pub _links: Links,
}

#[derive(Deserialize, Debug)]
pub struct CastCredit {
    #[serde(rename = "self")]
    pub _self: bool,
    pub voice: bool,
    pub _links: Links,
}
