/// The requested time range of Transactions are provided.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Idrange {
    /// The list of Transactions that satisfy the request.
    transactions: Vec<Transaction>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Idrange {
    fn default() -> Self {
        Self {
            transactions: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
