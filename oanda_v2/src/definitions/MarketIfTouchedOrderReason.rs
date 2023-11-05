/// The reason that the Market-if-touched Order was initiated
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum MarketIfTouchedOrderReason {
    /// The Market-if-touched Order was initiated at the request of
    /// a client
    ClientOrder,
    /// The Market-if-touched Order was initiated as a replacement
    /// for an existing Order
    Replacement,
}
