use std::fmt::Display;

mod models;

mod search;
pub use search::*;
mod schedule;
pub use schedule::*;
mod shows;
pub use shows::*;
mod episodes;
pub use episodes::*;
mod people;
pub use people::*;
mod updates;
pub use updates::*;

macro_rules! notfoundable_endpoint {
    ($url:expr) => {{
        let res = reqwest::get($url).await?;
        Ok(match res.status() {
            StatusCode::NOT_FOUND => None,
            _ => Some(serde_json::from_str(&res.text().await?).unwrap()),
        })
    }};
}

pub(crate) use notfoundable_endpoint;

pub struct Embed {
    pub episodes: bool,
    pub cast: bool,
    pub next_episode: bool,
}

impl Display for Embed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &[
                if self.episodes {
                    "embed[]=episodes"
                } else {
                    ""
                },
                if self.cast { "embed[]=cast" } else { "" },
                if self.next_episode {
                    "embed[]=nextepisode"
                } else {
                    ""
                },
            ]
            .join("&"),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    pub const TEST_SHOWS: [(&str, usize, usize); 5] = [
        ("Man Vs Bee", 52430, 393434),
        ("Alchemy of Souls", 62248, 401475),
        ("Westworld", 1371, 296762),
        ("Riviera", 16077, 326559),
        ("Stranger Things", 2993, 305288),
    ];

    pub const FULL_EMBED: Embed = Embed {
        episodes: true,
        cast: true,
        next_episode: true,
    };

    pub const TEST_ACTORS: [&str; 5] = [
        "Tom Hanks",
        "Denzel Washington",
        "Samuel L. Jackson",
        "Morgan Freeman",
        "Harrison Ford",
    ];
}
