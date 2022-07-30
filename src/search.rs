use std::fmt::Display;

use reqwest::StatusCode;

use crate::{
    models::{SearchResult, SearchResultPerson, Show},
    Embed,
};

pub enum ExternId {
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

/// Search through all the shows in our database by the show's name. A fuzzy algorithm is used (with a fuzziness value of 2), meaning that shows will be found even if your query contains small typos. Results are returned in order of relevancy (best matches on top) and contain each show's full information.
pub async fn show_search(query: &str) -> Result<Vec<SearchResult>, reqwest::Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/search/shows?q={}", query))
            .await?
            .text()
            .await?,
    )
    .unwrap())
}

/// In some scenarios you might want to immediately return information based on a user's query, without the intermediary step of presenting them all the possible matches. In that case, you can use the singlesearch endpoint which either returns exactly one result, or no result at all. This endpoint is also forgiving of typos, but less so than the regular search (with a fuzziness of 1 instead of 2), to reduce the chance of a false positive.
pub async fn show_single_search(query: &str, embed: Embed) -> Result<Option<Show>, reqwest::Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "https://api.tvmaze.com/singlesearch/shows?q={}&{}",
            query, embed
        ))
        .await?
        .text()
        .await?,
    )
    .unwrap())
}

/// If you already know a show's tvrage, thetvdb or IMDB ID, you can use this endpoint to find this exact show on TVmaze. If the given ID can be matched, a HTTP 302 redirect to the show's URL will be returned. Otherwise, a HTTP 404 is sent.
pub async fn show_lookup(extern_id: ExternId) -> Result<Option<Show>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!("https://api.tvmaze.com/lookup/shows?{}", extern_id))
}

/// Search through all the people in our database, using the same mechanism as described for show searches.
pub async fn people_search(query: &str) -> Result<Vec<SearchResultPerson>, reqwest::Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/search/people?q={}", query))
            .await?
            .text()
            .await?,
    )
    .unwrap())
}

#[cfg(test)]
mod tests {
    use crate::{
        search::{show_lookup, show_search, show_single_search, ExternId},
        tests::{FULL_EMBED, TEST_ACTORS, TEST_SHOWS},
    };

    use super::people_search;

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
            let res = show_single_search(name, FULL_EMBED).await.unwrap().unwrap();

            assert_eq!(res.name, name);
            assert_eq!(res.id, id);
            assert_eq!(res.externals.thetvdb.unwrap(), tvdb);
        }
    }

    #[tokio::test]
    async fn test_show_lookup() {
        for (name, id, tvdb) in TEST_SHOWS {
            let res = show_lookup(ExternId::TVDB(tvdb)).await.unwrap().unwrap();

            assert_eq!(res.name, name);
            assert_eq!(res.id, id);
            assert_eq!(res.externals.thetvdb.unwrap(), tvdb);
        }
    }

    #[tokio::test]
    async fn test_people_search() {
        for x in TEST_ACTORS {
            people_search(x).await.unwrap();
        }
    }
}
