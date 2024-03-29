use serde::{Serialize, Deserialize};
/// DateTime<Utc> header
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AcceptDatetimeFormat {
    /// If “UNIX” is specified DateTime<Utc> fields will be specified or
    /// returned in the “12345678.000000123” format.
    Unix,
    /// If “RFC3339” is specified DateTime<Utc> will be specified or
    /// returned in “YYYY-MM-DDTHH:MM:SS.nnnnnnnnnZ” format.
    Rfc3339,
}
