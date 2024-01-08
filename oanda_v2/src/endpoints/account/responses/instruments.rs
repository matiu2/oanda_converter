use definitions::instrument::Instrument;
use definitions::transaction_id::TransactionID;
use serde::{Serialize, Deserialize};
/// The list of tradeable instruments for the Account has been
/// provided.
use serde::{Serialize, Deserialize};
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
        use Default::default;
        Self {
            instruments: default(),
            last_transaction_id: default(),
        }
    }
}
