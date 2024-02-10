use crate::definitions::transaction_filter::TransactionFilter;
use chrono::Utc;
use serde_inline_default::serde_inline_default;
use chrono::DateTime;
use crate::definitions::transaction_id::TransactionID;
/// The requested time range of Transaction pages are provided.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Transactions {
    /// The starting time provided in the request.
    from: Option<DateTime<Utc>>,
    /// The ending time provided in the request.
    to: Option<DateTime<Utc>>,
    /// The pageSize provided in the request
    page_size: Option<integer>,
    /// The Transaction-type filter provided in the request
    r#type: Vec<TransactionFilter>,
    /// The number of Transactions that are contained in the pages
    /// returned
    count: Option<integer>,
    /// The list of URLs that represent idrange queries providing
    /// the data for each page in the query results
    pages: Vec<String>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Transactions {
    fn default() -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
            page_size: Default::default(),
            r#type: Default::default(),
            count: Default::default(),
            pages: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
