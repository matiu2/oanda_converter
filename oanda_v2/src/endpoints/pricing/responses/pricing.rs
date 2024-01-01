use definitions::home_conversions::HomeConversions;
use definitions::client_price::ClientPrice;
use chrono::DateTime;
use serde::{Serialize, Deserialize};
/// Pricing information has been successfully provided.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Pricing200 {
    /// The list of Price objects requested.
    prices: Vec<ClientPrice>,
    /// The list of home currency conversion factors requested. This
    /// field will only be present if includeHomeConversions was set
    /// to true in the request.
    home_conversions: Vec<HomeConversions>,
    /// The DateTime value to use for the “since” parameter in the
    /// next poll request.
    time: Option<DateTime>,
}
impl Default for Pricing200 {
    fn default() -> Self {
        use Default::default;
        Self {
            prices: default(),
            home_conversions: default(),
            time: default(),
        }
    }
}
