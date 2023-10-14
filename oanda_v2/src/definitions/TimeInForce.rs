/// The time-in-force of an Order. TimeInForce describes how long an Order should remain pending before being automatically cancelled by the execution system.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    /// The Order is “Good unTil Cancelled”
    Gtc,
    /// The Order is “Good unTil Date” and will be cancelled at the provided time
    Gtd,
    /// The Order is “Good For Day” and will be cancelled at 5pm New York time
    Gfd,
    /// The Order must be immediately “Filled Or Killed”
    Fok,
    /// The Order must be “Immediately partially filled Or Cancelled”
    Ioc,
}
