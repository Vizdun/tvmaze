use serde::Deserialize;

use super::{people::Person, shows::Show};

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    pub score: f32,
    pub show: Show,
}

#[derive(Deserialize, Debug)]
pub struct SearchResultPerson {
    pub score: f32,
    pub person: Person,
}
