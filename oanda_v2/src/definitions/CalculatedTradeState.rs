use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct CalculatedTradeState {
    /// The Trade’s ID.
    id: Option<TradeID>,
    /// The Trade’s unrealized profit/loss.
    unrealized_pl: Option<AccountUnits>,
    /// Margin currently used by the Trade.
    margin_used: Option<AccountUnits>,
}
