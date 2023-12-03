/// The details of the requested Transaction are provided.
#[derive(Serialize, Deserialize)]
struct Get200 {
    /// The details of the Transaction requested
    transaction: Option<Transaction>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
