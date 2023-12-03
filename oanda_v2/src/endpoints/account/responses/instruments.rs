/// The list of tradeable instruments for the Account has been
/// provided.
#[derive(Serialize, Deserialize)]
struct Instruments200 {
    /// The requested list of instruments.
    instruments: Vec<Instrument>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
}
