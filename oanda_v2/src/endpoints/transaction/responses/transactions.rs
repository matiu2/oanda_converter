use serde::{Serialize, Deserialize};
/// The requested time range of Transaction pages are provided.
#[derive(Serialize, Deserialize)]
struct Transactions200 {
    /// The starting time provided in the request.
    from: Option<DateTime>,
    /// The ending time provided in the request.
    to: Option<DateTime>,
    /// The pageSize provided in the request
    page_size: Option<integer>,
    /// The Transaction-type filter provided in the request
    r#type: Vec<TransactionFilter>,
    /// The number of Transactions that are contained in the pages
    /// returned
    count: Option<integer>,
    /// The list of URLs that represent idrange queries providing
    /// the data for each page in the query results
    pages: Vec<string>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
