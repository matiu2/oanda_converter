use crate::definitions::instrument::Instrument;
use crate::definitions::transaction_id::TransactionID;
/// The list of tradeable instruments for the Account has been
/// provided.
#[derive(Serialize, Deserialize)]
pub struct Instruments {
    /// The requested list of instruments.
    instruments: Vec<Instrument>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
}
impl Default for Instruments {
    fn default() -> Self {
        Self {
            instruments: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
