use lib::endpoints::trade;
use serde::{Serialize, Deserialize};
/// The Account’s list of open Trades is provided
#[derive(Serialize, Deserialize)]
struct OpenTrades200 {
    /// The Account’s list of open Trades
    trades: Vec<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
