use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct CalculatedTradeState {
    /// The Trade’s ID.
    id: Option<TradeID>,
    /// The Trade’s unrealized profit/loss.
    unrealizedPL: Option<AccountUnits>,
    /// Margin currently used by the Trade.
    marginUsed: Option<AccountUnits>,
}
