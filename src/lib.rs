use std::{collections::HashMap, fmt::Display};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SearchResult {
    score: f32,
    show: Show,
}

#[derive(Deserialize, Debug)]
struct SearchResultPerson {
    score: f32,
    person: Person,
}

#[derive(Deserialize, Debug)]
struct Show {
    id: usize,
    url: String,
    name: String,
    r#type: String,
    language: String,
    genres: Vec<String>,
    status: String,
    runtime: Option<usize>,
    averageRuntime: Option<usize>,
    premiered: Option<String>,
    ended: Option<String>,
    officialSite: Option<String>,
    schedule: Schedule,
    rating: Rating,
    weight: usize,
    network: Option<Network>,
    webChannel: Option<Network>,
    dvdCountry: Option<Country>,
    externals: Externals,
    image: Option<Image>,
    summary: Option<String>,
    updated: u64,
    _links: Links,
    _embedded: Option<Embedded>,
}

#[derive(Deserialize, Debug)]
struct Schedule {
    time: String,
    days: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Rating {
    average: Option<f32>,
}

#[derive(Deserialize, Debug)]
struct Network {
    id: usize,
    name: String,
    country: Option<Country>,
    officialSite: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Country {
    name: String,
    code: String,
    timezone: String,
}

#[derive(Deserialize, Debug)]
struct Externals {
    tvrage: Option<usize>,
    thetvdb: Option<usize>,
    imdb: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Image {
    medium: String,
    original: String,
}

#[derive(Deserialize, Debug)]
struct Links {
    #[serde(rename = "self")]
    _self: Option<Hrefed>,
    previousepisode: Option<Hrefed>,
}

#[derive(Deserialize, Debug)]
struct Hrefed {
    href: String,
}

#[derive(Deserialize, Debug)]
struct Embedded {
    episodes: Option<Vec<Episode>>,
    cast: Option<Vec<CastMember>>,
    nextepisode: Option<Episode>,
}

#[derive(Deserialize, Debug)]
struct Episode {
    id: usize,
    url: String,
    name: String,
    season: usize,
    number: Option<usize>,
    r#type: String,
    airdate: String,
    airtime: String,
    airstamp: String,
    runtime: Option<usize>,
    rating: Rating,
    image: Option<Image>,
    summary: Option<String>,
    _links: Links,
}

#[derive(Deserialize, Debug)]
struct CastMember {
    person: Person,
    character: Character,
    #[serde(rename = "self")]
    _self: bool,
    voice: bool,
}

#[derive(Deserialize, Debug)]
struct Person {
    id: usize,
    url: String,
    name: String,
    country: Option<Country>,
    birthday: Option<String>,
    deathday: Option<String>,
    gender: Option<String>,
    image: Option<Image>,
    updated: usize,
    _links: Links,
}

#[derive(Deserialize, Debug)]
struct Character {
    id: usize,
    url: String,
    name: String,
    image: Option<Image>,
    _links: Links,
}

#[derive(Deserialize, Debug)]
struct AlternateList {
    id: usize,
    url: String,
    dvd_release: bool,
    verbatim_order: bool,
    country_premiere: bool,
    streaming_premiere: bool,
    broadcast_premiere: bool,
    language_premiere: bool,
    language: Option<String>,
    network: Option<Network>,
    webChannel: Option<Network>,
    _links: Links,
}

#[derive(Deserialize, Debug)]
struct Season {
    id: usize,
    url: String,
    number: usize,
    name: String,
    episodeOrder: Option<usize>,
    premiereDate: Option<String>,
    endDate: Option<String>,
    network: Option<Network>,
    webChannel: Option<Network>,
    image: Option<Image>,
    summary: Option<String>,
    _links: Links,
}

#[derive(Deserialize, Debug)]
struct CrewMember {
    r#type: String,
    person: Person,
}

#[derive(Deserialize, Debug)]
struct Aka {
    name: String,
    country: Option<Country>,
}

#[derive(Deserialize, Debug)]
struct ShowImage {
    id: usize,
    r#type: String,
    main: bool,
    resolutions: Resolutions,
}

#[derive(Deserialize, Debug)]
struct Resolutions {
    original: Resolution,
    medium: Option<Resolution>,
}

#[derive(Deserialize, Debug)]
struct Resolution {
    url: String,
    width: usize,
    height: usize,
}

#[derive(Deserialize, Debug)]
struct CrewCredit {
    r#type: String,
    _links: Links,
}

#[derive(Deserialize, Debug)]
struct CastCredit {
    #[serde(rename = "self")]
    _self: bool,
    voice: bool,
    _links: Links,
}

#[derive(Debug)]
enum Error {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(e)
    }
}

#[derive(Default)]
struct Embed {
    episodes: bool,
    cast: bool,
    next_episode: bool,
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

async fn show_search(query: &str) -> Result<Vec<SearchResult>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/search/shows?q={}", query))
            .await?
            .text()
            .await?,
    )?)
}

async fn show_single_search(query: &str, embed: Embed) -> Result<Show, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "https://api.tvmaze.com/singlesearch/shows?q={}&{}",
            query, embed
        ))
        .await?
        .text()
        .await?,
    )?)
}

enum ExternId {
    TVRage(usize),
    TVDB(usize),
    IMDB(usize),
}

impl Display for ExternId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            ExternId::TVRage(i) => format!("tvrage={}", i),
            ExternId::TVDB(i) => format!("thetvdb={}", i),
            ExternId::IMDB(i) => format!("imdb=tt{}", i),
        })
    }
}

async fn show_lookup(extern_id: ExternId) -> Result<Show, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/lookup/shows?{}", extern_id))
            .await?
            .text()
            .await?,
    )?)
}

async fn people_search(query: &str) -> Result<Vec<SearchResultPerson>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/search/people?q={}", query))
            .await?
            .text()
            .await?,
    )?)
}

// TODO: params are optional
async fn schedule(country_code: &str, date: &str) -> Result<Vec<Episode>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "https://api.tvmaze.com/schedule?country={}&date={}",
            country_code, date
        ))
        .await?
        .text()
        .await?,
    )?)
}

// TODO: params are optional, country_code has special fn if empty
async fn web_schedule(country_code: &str, date: &str) -> Result<Vec<Episode>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "https://api.tvmaze.com/schedule/web?country={}&date={}",
            country_code, date
        ))
        .await?
        .text()
        .await?,
    )?)
}

async fn full_schedule() -> Result<Vec<Episode>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get("https://api.tvmaze.com/schedule/full")
            .await?
            .text()
            .await?,
    )?)
}

async fn show(id: usize, embed: Embed) -> Result<Show, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/shows/{}?{}", id, embed))
            .await?
            .text()
            .await?,
    )?)
}

async fn show_episode_list(show: usize, specials: bool) -> Result<Vec<Episode>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "https://api.tvmaze.com/shows/{}/episodes{}",
            show,
            if specials { "?specials=1" } else { "" }
        ))
        .await?
        .text()
        .await?,
    )?)
}

// TODO: alternate lists embed
// TODO: all the other alternate list endpoints
async fn show_alternate_lists(show: usize) -> Result<Vec<AlternateList>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "https://api.tvmaze.com/shows/{}/alternatelists",
            show,
        ))
        .await?
        .text()
        .await?,
    )?)
}

async fn show_seasons(show: usize) -> Result<Vec<Season>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/shows/{}/seasons", show,))
            .await?
            .text()
            .await?,
    )?)
}

async fn season_episodes(season: usize) -> Result<Vec<Episode>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "https://api.tvmaze.com/seasons/{}/episodes",
            season,
        ))
        .await?
        .text()
        .await?,
    )?)
}

async fn show_cast(show: usize) -> Result<Vec<CastMember>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/shows/{}/cast", show,))
            .await?
            .text()
            .await?,
    )?)
}

async fn show_crew(show: usize) -> Result<Vec<CrewMember>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/shows/{}/crew", show,))
            .await?
            .text()
            .await?,
    )?)
}

async fn show_akas(show: usize) -> Result<Vec<Aka>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/shows/{}/akas", show,))
            .await?
            .text()
            .await?,
    )?)
}

async fn show_images(show: usize) -> Result<Vec<ShowImage>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/shows/{}/images", show,))
            .await?
            .text()
            .await?,
    )?)
}

async fn show_index(page: usize) -> Result<Vec<Show>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/shows?page={}", page,))
            .await?
            .text()
            .await?,
    )?)
}

async fn episodes_by_date(show: usize, date: &str) -> Result<Vec<Episode>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "https://api.tvmaze.com/shows/{}/episodesbydate?date={}",
            show, date
        ))
        .await?
        .text()
        .await?,
    )?)
}

async fn episode_by_number(show: usize, season: usize, episode: usize) -> Result<Episode, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "https://api.tvmaze.com/shows/{}/episodebynumber?season={}&number={}",
            show, season, episode,
        ))
        .await?
        .text()
        .await?,
    )?)
}

async fn episode(id: usize) -> Result<Episode, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/episodes/{}", id,))
            .await?
            .text()
            .await?,
    )?)
}

async fn person(id: usize) -> Result<Person, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/people/{}", id,))
            .await?
            .text()
            .await?,
    )?)
}

async fn person_cast_credits(id: usize) -> Result<Vec<CastCredit>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/people/{}/castcredits", id,))
            .await?
            .text()
            .await?,
    )?)
}

async fn person_crew_credits(id: usize) -> Result<Vec<CrewCredit>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/people/{}/crewcredits", id,))
            .await?
            .text()
            .await?,
    )?)
}

async fn person_index(page: usize) -> Result<Vec<Person>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/people?page={}", page,))
            .await?
            .text()
            .await?,
    )?)
}

async fn show_updates() -> Result<HashMap<usize, usize>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/updates/shows"))
            .await?
            .text()
            .await?,
    )?)
}

async fn person_updates() -> Result<HashMap<usize, usize>, Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/updates/people"))
            .await?
            .text()
            .await?,
    )?)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_SHOWS: [(&str, usize, usize); 5] = [
        ("Man Vs Bee", 52430, 393434),
        ("Alchemy of Souls", 62248, 401475),
        ("Westworld", 1371, 296762),
        ("Riviera", 16077, 326559),
        ("Stranger Things", 2993, 305288),
    ];

    const FULL_EMBED: Embed = Embed {
        episodes: true,
        cast: true,
        next_episode: true,
    };

    #[tokio::test]
    async fn test_show_search() {
        for (name, id, tvdb) in TEST_SHOWS {
            let res = &show_search(name).await.unwrap()[0].show;

            assert_eq!(res.name, name);
            assert_eq!(res.id, id);
            assert_eq!(res.externals.thetvdb.unwrap(), tvdb);
        }
    }

    #[tokio::test]
    async fn test_show_single_search() {
        for (name, id, tvdb) in TEST_SHOWS {
            let res = show_single_search(name, FULL_EMBED).await.unwrap();

            assert_eq!(res.name, name);
            assert_eq!(res.id, id);
            assert_eq!(res.externals.thetvdb.unwrap(), tvdb);
        }
    }

    #[tokio::test]
    async fn test_show_lookup() {
        for (name, id, tvdb) in TEST_SHOWS {
            let res = show_lookup(ExternId::TVDB(tvdb)).await.unwrap();

            assert_eq!(res.name, name);
            assert_eq!(res.id, id);
            assert_eq!(res.externals.thetvdb.unwrap(), tvdb);
        }
    }

    #[tokio::test]
    async fn test_show() {
        for (name, id, tvdb) in TEST_SHOWS {
            let res = show(id, FULL_EMBED).await.unwrap();

            assert_eq!(res.name, name);
            assert_eq!(res.id, id);
            assert_eq!(res.externals.thetvdb.unwrap(), tvdb);
        }
    }

    const TEST_ACTORS: [&str; 5] = [
        "Tom Hanks",
        "Denzel Washington",
        "Samuel L. Jackson",
        "Morgan Freeman",
        "Harrison Ford",
    ];

    #[tokio::test]
    async fn test_people_search() {
        for x in TEST_ACTORS {
            people_search(x).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_schedule() {
        schedule("US", "2014-12-01").await.unwrap();
    }

    #[tokio::test]
    async fn test_web_schedule() {
        web_schedule("US", "2020-05-29").await.unwrap();
    }

    #[tokio::test]
    async fn test_full_schedule() {
        return;
        full_schedule().await.unwrap();
    }

    #[tokio::test]
    async fn test_show_episode_list() {
        assert_eq!(show_episode_list(204, true).await.unwrap().len(), 218)
    }

    #[tokio::test]
    async fn test_show_alternate_lists() {
        show_alternate_lists(180).await.unwrap();
    }

    #[tokio::test]
    async fn test_episode_by_number() {
        assert_eq!(
            "Apotheosis".to_string(),
            episode_by_number(433, 1, 18).await.unwrap().name
        )
    }

    #[tokio::test]
    async fn test_episodes_by_date() {
        assert_eq!(
            "A Christmas Carol".to_string(),
            episodes_by_date(210, "2010-12-25").await.unwrap()[0].name
        )
    }

    #[tokio::test]
    async fn test_episode() {
        assert_eq!("Pilot".to_string(), episode(1).await.unwrap().name)
    }

    #[tokio::test]
    async fn test_show_seasons() {
        for (_, id, _) in TEST_SHOWS {
            show_seasons(id).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_season_episodes() {
        for id in 1..10 {
            season_episodes(id).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_show_cast() {
        for (_, id, _) in TEST_SHOWS {
            show_cast(id).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_show_crew() {
        for (_, id, _) in TEST_SHOWS {
            show_crew(id).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_show_akas() {
        for (_, id, _) in TEST_SHOWS {
            show_akas(id).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_show_images() {
        for (_, id, _) in TEST_SHOWS {
            show_images(id).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_show_index() {
        let res = &show_index(0).await.unwrap()[0];
        assert!(res.id <= 250);
    }

    #[tokio::test]
    async fn test_person() {
        person(1).await.unwrap();
    }

    #[tokio::test]
    async fn test_person_cast_credits() {
        person_cast_credits(1).await.unwrap();
    }

    #[tokio::test]
    async fn test_person_crew_credits() {
        person_crew_credits(1).await.unwrap();
    }

    #[tokio::test]
    async fn test_person_index() {
        let res = &person_index(0).await.unwrap()[0];
        assert!(res.id <= 250);
    }

    #[tokio::test]
    async fn test_show_updates() {
        show_updates().await.unwrap();
    }

    #[tokio::test]
    async fn test_person_updates() {
        person_updates().await.unwrap();
    }
}
