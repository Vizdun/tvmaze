use crate::models::Episode;

// TODO: params are optional
/// The schedule is a complete list of episodes that air in a given country on a given date. Episodes are returned in the order in which they are aired, and full information about the episode and the corresponding show is included.
pub async fn schedule(country_code: &str, date: &str) -> Result<Vec<Episode>, reqwest::Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "https://api.tvmaze.com/schedule?country={}&date={}",
            country_code, date
        ))
        .await?
        .text()
        .await?,
    )
    .unwrap())
}

// TODO: params are optional, country_code has special fn if empty
/// The web schedule is a complete list of episodes that air on web/streaming channels on a given date. TVmaze distinguishes between local and global Web Channels: local Web Channels are only available in one specific country, while global Web Channels are available in multiple countries. To query both local and global Web Channels, leave out the country parameter. To query only local Web Channels, set country to an ISO country code. And to query only global Web Channels, set country to an empty string.
pub async fn web_schedule(country_code: &str, date: &str) -> Result<Vec<Episode>, reqwest::Error> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "https://api.tvmaze.com/schedule/web?country={}&date={}",
            country_code, date
        ))
        .await?
        .text()
        .await?,
    )
    .unwrap())
}

/// The full schedule is a list of all future episodes known to TVmaze, regardless of their country. Be advised that this endpoint's response is at least several MB large. As opposed to the other endpoints, results are cached for 24 hours.
pub async fn full_schedule() -> Result<Vec<Episode>, reqwest::Error> {
    Ok(serde_json::from_str(
        &reqwest::get("https://api.tvmaze.com/schedule/full")
            .await?
            .text()
            .await?,
    )
    .unwrap())
}

#[cfg(test)]
mod tests {
    use super::{full_schedule, schedule, web_schedule};

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
        full_schedule().await.unwrap();
    }
}
