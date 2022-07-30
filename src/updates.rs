use std::collections::HashMap;

/// A list of all shows in the TVmaze database and the timestamp when they were last updated. Updating a direct or indirect child of a show will also mark the show itself as updated. For example; creating, deleting or updating an episode or an episode's gallery item will mark the episode's show as updated. It's possible to filter the resultset to only include shows that have been updated in the past day (24 hours), week, or month.
pub async fn show_updates() -> Result<HashMap<usize, usize>, reqwest::Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/updates/shows"))
            .await?
            .text()
            .await?,
    )
    .unwrap())
}

/// Like the show updates endpoint, but for people. A person is considered to be updated when any of their attributes are changed, but also when a cast- or crew-credit that involves them is created or deleted.
pub async fn person_updates() -> Result<HashMap<usize, usize>, reqwest::Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!("https://api.tvmaze.com/updates/people"))
            .await?
            .text()
            .await?,
    )
    .unwrap())
}

#[cfg(test)]
mod tests {
    use super::{person_updates, show_updates};

    #[tokio::test]
    async fn test_show_updates() {
        show_updates().await.unwrap();
    }

    #[tokio::test]
    async fn test_person_updates() {
        person_updates().await.unwrap();
    }
}
