use definitions::account_units::AccountUnits;
use definitions::trade_id::TradeID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct CalculatedTradeState {
    /// The Trade’s ID.
    id: Option<TradeID>,
    /// The Trade’s unrealized profit/loss.
    unrealized_pl: Option<AccountUnits>,
    /// Margin currently used by the Trade.
    margin_used: Option<AccountUnits>,
}
impl Default for CalculatedTradeState {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            unrealized_pl: default(),
            margin_used: default(),
        }
    }
}
