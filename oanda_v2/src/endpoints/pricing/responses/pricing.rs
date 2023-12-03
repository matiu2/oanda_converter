/// Pricing information has been successfully provided.
#[derive(Serialize, Deserialize)]
struct Pricing200 {
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
