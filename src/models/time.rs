use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use serde::Deserialize;

// struct CustomVisitor();

// impl<'de> Visitor<'de> for CustomVisitor {
//     type Value = String;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         write!(formatter, "serde trickery")
//     }

//     fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         Ok(s.to_owned())
//     }
// }

#[derive(Deserialize, Debug)]
#[serde(from = "String")]
pub struct TVMazeDateTime(pub chrono::DateTime<FixedOffset>);

impl From<String> for TVMazeDateTime {
    fn from(s: String) -> Self {
        TVMazeDateTime(DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%:z").unwrap())
    }
}

#[derive(Deserialize, Debug)]
#[serde(from = "String")]
pub struct TVMazeTime(pub Option<chrono::NaiveTime>);

impl From<String> for TVMazeTime {
    fn from(s: String) -> Self {
        TVMazeTime(NaiveTime::parse_from_str(&s, "%H:%M").ok())
    }
}

#[derive(Deserialize, Debug)]
#[serde(from = "String")]
pub struct TVMazeDate(pub chrono::NaiveDate);

impl From<String> for TVMazeDate {
    fn from(s: String) -> Self {
        TVMazeDate(NaiveDate::parse_from_str(&s, "%Y-%m-%d").unwrap())
    }
}
