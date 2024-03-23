/// The list of Trades requested
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Trades {
    /// The list of Trade detail objects
    trades: Vec<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Trades {
    fn default() -> Self {
        Self {
            trades: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
