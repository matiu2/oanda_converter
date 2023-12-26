/// The list of Trades requested
#[derive(Serialize, Deserialize)]
struct Trades200 {
    /// The list of Trade detail objects
    trades: Vec<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}