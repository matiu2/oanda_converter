/// The Position is provided.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Get {
    /// The requested Position.
    position: Option<Position>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Get {
    fn default() -> Self {
        Self {
            position: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
