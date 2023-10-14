/// The state to filter the requested Orders by.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum OrderStateFilter {
    /// The Orders that are currently pending execution
    Pending,
    /// The Orders that have been filled
    Filled,
    /// The Orders that have been triggered
    Triggered,
    /// The Orders that have been cancelled
    Cancelled,
    /// The Orders that are in any of the possible states listed above
    All,
}
