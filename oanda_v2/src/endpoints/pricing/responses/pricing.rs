use chrono::DateTime<Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc>;
use crate::definitions::client_price::ClientPrice;
use chrono::Utc;
use crate::definitions::home_conversions::HomeConversions;
/// Pricing information has been successfully provided.
#[derive(Serialize, Deserialize)]
pub struct Pricing200 {
    /// The list of Price objects requested.
    prices: Vec<ClientPrice>,
    /// The list of home currency conversion factors requested. This
    /// field will only be present if includeHomeConversions was set
    /// to true in the request.
    home_conversions: Vec<HomeConversions>,
    /// The DateTime<Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc> value to use for the “since” parameter in the
    /// next poll request.
    time: Option<DateTime<Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc><Utc>>,
}
impl Default for Pricing200 {
    fn default() -> Self {
        Self {
            prices: Default::default(),
            home_conversions: Default::default(),
            time: Default::default(),
        }
    }
}
