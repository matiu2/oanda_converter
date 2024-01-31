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
        Self {
            id: Default::default(),
            unrealized_pl: Default::default(),
            margin_used: Default::default(),
        }
    }
}
