use reqwest::StatusCode;

use crate::models::Episode;

/// Retrieve all primary information for a given episode. This endpoint allows embedding of additional information. See the section embedding for more information.
pub async fn episode(id: usize) -> Result<Option<Episode>, reqwest::Error> {
    crate::notfoundable_endpoint!(format!("https://api.tvmaze.com/episodes/{}", id,))
}

#[cfg(test)]
mod tests {
    use super::episode;

    #[tokio::test]
    async fn test_episode() {
        episode(1).await.unwrap().unwrap();
    }
}
