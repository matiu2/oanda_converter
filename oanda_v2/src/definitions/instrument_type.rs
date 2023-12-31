use serde::{Serialize, Deserialize};
/// The type of an Instrument.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentType {
    /// Currency
    Currency,
    /// Contract For Difference
    Cfd,
    /// Metal
    Metal,
}
