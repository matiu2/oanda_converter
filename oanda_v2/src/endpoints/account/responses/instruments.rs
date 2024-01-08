use crate::definitions::transaction_id::TransactionID;
use crate::definitions::instrument::Instrument;
/// The list of tradeable instruments for the Account has been
/// provided.
#[derive(Serialize, Deserialize)]
pub struct Instruments200 {
    /// The requested list of instruments.
    instruments: Vec<Instrument>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
}
impl Default for Instruments200 {
    fn default() -> Self {
        Self {
            instruments: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
