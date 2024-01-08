use crate::endpoints::order::Order;
use crate::endpoints::position::Position;
use crate::endpoints::transaction::Transaction;
use crate::definitions::trade_summary::TradeSummary;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct AccountChanges {
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
impl Default for AccountChanges {
    fn default() -> Self {
        Self {
            orders_created: Default::default(),
            orders_cancelled: Default::default(),
            orders_filled: Default::default(),
            orders_triggered: Default::default(),
            trades_opened: Default::default(),
            trades_reduced: Default::default(),
            trades_closed: Default::default(),
            positions: Default::default(),
            transactions: Default::default(),
        }
    }
}
