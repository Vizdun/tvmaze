use serde::Deserialize;

use super::{
    time::{TVMazeDate, TVMazeDateTime, TVMazeTime},
    Image, Links, Rating,
};

#[derive(Deserialize, Debug)]
pub struct Episode {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub season: usize,
    pub number: Option<usize>,
    pub r#type: String,
    pub airdate: TVMazeDate,
    pub airtime: TVMazeTime,
    pub airstamp: TVMazeDateTime,
    pub runtime: Option<usize>,
    pub rating: Rating,
    pub image: Option<Image>,
    pub summary: Option<String>,
    pub _links: Links,
}
