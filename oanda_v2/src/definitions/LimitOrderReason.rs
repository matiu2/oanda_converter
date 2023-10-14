/// The reason that the Limit Order was initiated
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum LimitOrderReason {
    /// The Limit Order was initiated at the request of a client
    ClientOrder,
    /// The Limit Order was initiated as a replacement for an existing Order
    Replacement,
}
