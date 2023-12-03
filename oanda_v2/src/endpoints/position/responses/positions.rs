/// The Account’s Positions are provided.
#[derive(Serialize, Deserialize)]
struct Positions200 {
    /// The list of Account Positions.
    positions: Vec<Position>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
