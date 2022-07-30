use serde::Deserialize;

mod episodes;
pub use episodes::*;
mod people;
pub use people::*;
mod shows;
pub use shows::*;
mod search;
pub use search::*;
mod time;

#[derive(Deserialize, Debug)]
pub struct Rating {
    pub average: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Country {
    pub name: String,
    pub code: String,
    pub timezone: String,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub medium: String,
    pub original: String,
}

#[derive(Deserialize, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    pub _self: Option<Hrefed>,
    pub previousepisode: Option<Hrefed>,
}

#[derive(Deserialize, Debug)]
pub struct Hrefed {
    pub href: String,
}
