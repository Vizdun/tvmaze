use reqwest::StatusCode;

use crate::models::{CastCredit, CrewCredit, Person};

/// Retrieve all primary information for a given person. This endpoint allows embedding of additional information. See the section embedding for more information.
pub async fn person(id: usize) -> Result<Option<Person>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!("https://api.tvmaze.com/people/{}", id,))
}

/// Retrieve all (show-level) cast credits for a person. A cast credit is a combination of both a show and a character. By default, only a reference to each show and character will be returned. However, this endpoint supports embedding, which means full information for the shows and characters can be included.
pub async fn person_cast_credits(id: usize) -> Result<Option<Vec<CastCredit>>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!("https://api.tvmaze.com/people/{}/castcredits", id,))
}

/// Retrieve all (show-level) crew credits for a person. A crew credit is combination of both a show and a crew type. By default, only a reference to each show will be returned. However, this endpoint supports embedding, which means full information for the shows can be included.
pub async fn person_crew_credits(id: usize) -> Result<Option<Vec<CrewCredit>>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!("https://api.tvmaze.com/people/{}/crewcredits", id,))
}

/// Like the show index but for people; please refer to the show index documentation. A maximum of 1000 results per page is returned.
pub async fn person_index(page: usize) -> Result<Vec<Person>, reqwest::Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/people?page={}", page,))
            .await?
            .text()
            .await?,
    )
    .unwrap())
}

#[cfg(test)]
mod tests {
    use super::{person, person_cast_credits, person_crew_credits, person_index};

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
}
