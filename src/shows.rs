use reqwest::StatusCode;

use crate::{
    models::{Aka, AlternateList, CastMember, CrewMember, Episode, Season, Show, ShowImage},
    Embed,
};

/// Retrieve all primary information for a given show. This endpoint allows embedding of additional information. See the section embedding for more information.
pub async fn show(id: usize, embed: Embed) -> Result<Option<Show>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!("https://api.tvmaze.com/shows/{}?{}", id, embed))
}

/// A complete list of episodes for the given show. Episodes are returned in their airing order, and include full episode information. By default, specials are not included in the list.
pub async fn show_episode_list(
    show: usize,
    specials: bool,
) -> Result<Option<Vec<Episode>>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!(
        "https://api.tvmaze.com/shows/{}/episodes{}",
        show,
        if specials { "?specials=1" } else { "" }
    ))
}

// TODO: alternate lists embed
// TODO: all the other alternate list endpoints
/// Alternate episode lists for this show, for example DVD ordering. For a description of the different types of alternate lists that you can find, please refer to the alternate episode policy.
pub async fn show_alternate_lists(
    show: usize,
) -> Result<Option<Vec<AlternateList>>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!(
        "https://api.tvmaze.com/shows/{}/alternatelists",
        show,
    ))
}

/// Retrieve one specific episode from this show given its season number and episode number. This either returns the full information for one episode, or a HTTP 404.
pub async fn episode_by_number(
    show: usize,
    season: usize,
    episode: usize,
) -> Result<Option<Episode>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!(
        "https://api.tvmaze.com/shows/{}/episodebynumber?season={}&number={}",
        show, season, episode,
    ))
}

/// Retrieve all episodes from this show that have aired on a specific date. This either returns an array of full episode info, or a HTTP 404. Useful for daily (talk) shows that don't adhere to a common season numbering.
pub async fn episodes_by_date(
    show: usize,
    date: &str,
) -> Result<Option<Vec<Episode>>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!(
        "https://api.tvmaze.com/shows/{}/episodesbydate?date={}",
        show, date
    ))
}

/// A complete list of seasons for the given show. Seasons are returned in ascending order and contain the full information that's known about them.
pub async fn show_seasons(show: usize) -> Result<Option<Vec<Season>>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!("https://api.tvmaze.com/shows/{}/seasons", show,))
}

/// A list of episodes in this season. Specials are always included in this list.
pub async fn season_episodes(season: usize) -> Result<Option<Vec<Episode>>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!(
        "https://api.tvmaze.com/seasons/{}/episodes",
        season,
    ))
}

/// A list of main cast for a show. Each cast item is a combination of a person and a character. Items are ordered by importance, which is determined by the total number of appearances of the given character in this show.
pub async fn show_cast(show: usize) -> Result<Option<Vec<CastMember>>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!("https://api.tvmaze.com/shows/{}/cast", show,))
}

/// A list of main crew for a show. Each crew item is a combination of a person and their crew type.
pub async fn show_crew(show: usize) -> Result<Option<Vec<CrewMember>>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!("https://api.tvmaze.com/shows/{}/crew", show,))
}

/// A list of AKA's (aliases) for a show. An AKA with its country set to null indicates an AKA in the show's original country. Otherwise, it's the AKA for that show in the given foreign country.
pub async fn show_akas(show: usize) -> Result<Option<Vec<Aka>>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!("https://api.tvmaze.com/shows/{}/akas", show,))
}

/// A list of all images available for this show. The image type can be "poster", "banner", "background", "typography", or NULL in case of legacy unclassified images. For a definition of these types, please refer to the main image and general image policies.
pub async fn show_images(show: usize) -> Result<Option<Vec<ShowImage>>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!("https://api.tvmaze.com/shows/{}/images", show,))
}

/// A list of all shows in our database, with all primary information included. You can use this endpoint for example if you want to build a local cache of all shows contained in the TVmaze database. This endpoint is paginated, with a maximum of 250 results per page. The pagination is based on show ID, e.g. page 0 will contain shows with IDs between 0 and 250. This means a single page might contain less than 250 results, in case of deletions, but it also guarantees that deletions won't cause shuffling in the page numbering for other shows.
pub async fn show_index(page: usize) -> Result<Vec<Show>, reqwest::Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/shows?page={}", page,))
            .await?
            .text()
            .await?,
    )
    .unwrap())
}

#[cfg(test)]
mod tests {
    use crate::{
        shows::{show, show_index},
        tests::{FULL_EMBED, TEST_SHOWS},
    };

    use super::{
        episode_by_number, episodes_by_date, season_episodes, show_akas, show_alternate_lists,
        show_cast, show_crew, show_episode_list, show_images, show_seasons,
    };

    #[tokio::test]
    async fn test_show() {
        for (name, id, tvdb) in TEST_SHOWS {
            let res = show(id, FULL_EMBED).await.unwrap().unwrap();

            assert_eq!(res.name, name);
            assert_eq!(res.id, id);
            assert_eq!(res.externals.thetvdb.unwrap(), tvdb);
        }
    }

    #[tokio::test]
    async fn test_show_episode_list() {
        assert_eq!(
            show_episode_list(204, true).await.unwrap().unwrap().len(),
            218
        )
    }

    #[tokio::test]
    async fn test_show_alternate_lists() {
        show_alternate_lists(180).await.unwrap();
    }

    #[tokio::test]
    async fn test_episode_by_number() {
        assert_eq!(
            "Apotheosis".to_string(),
            episode_by_number(433, 1, 18).await.unwrap().unwrap().name
        )
    }

    #[tokio::test]
    async fn test_episodes_by_date() {
        assert_eq!(
            "A Christmas Carol".to_string(),
            episodes_by_date(210, "2010-12-25").await.unwrap().unwrap()[0].name
        )
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
}
