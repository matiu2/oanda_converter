use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct AccountChanges {
    /// The Orders created. These Orders may have been filled, cancelled or triggered in the same period.
    ordersCreated: Vec<Order>,
    /// The Orders cancelled.
    ordersCancelled: Vec<Order>,
    /// The Orders filled.
    ordersFilled: Vec<Order>,
    /// The Orders triggered.
    ordersTriggered: Vec<Order>,
    /// The Trades opened.
    tradesOpened: Vec<TradeSummary>,
    /// The Trades reduced.
    tradesReduced: Vec<TradeSummary>,
    /// The Trades closed.
    tradesClosed: Vec<TradeSummary>,
    /// The Positions changed.
    positions: Vec<Position>,
    /// The Transactions that have been generated.
    transactions: Vec<Transaction>,
}
