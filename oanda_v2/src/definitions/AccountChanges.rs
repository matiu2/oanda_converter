use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct AccountChanges {
    /// The Orders created. These Orders may have been filled,
    /// cancelled or triggered in the same period.
    orders_created: Vec<Order>,
    /// The Orders cancelled.
    orders_cancelled: Vec<Order>,
    /// The Orders filled.
    orders_filled: Vec<Order>,
    /// The Orders triggered.
    orders_triggered: Vec<Order>,
    /// The Trades opened.
    trades_opened: Vec<TradeSummary>,
    /// The Trades reduced.
    trades_reduced: Vec<TradeSummary>,
    /// The Trades closed.
    trades_closed: Vec<TradeSummary>,
    /// The Positions changed.
    positions: Vec<Position>,
    /// The Transactions that have been generated.
    transactions: Vec<Transaction>,
}
