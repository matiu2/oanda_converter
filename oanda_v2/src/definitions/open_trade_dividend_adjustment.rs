use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct OpenTradeDividendAdjustment {
    /// The ID of the Trade for which the dividend adjustment is to
    /// be paid or collected.
    trade_id: Option<TradeID>,
    /// The dividend adjustment amount to pay or collect for the
    /// Trade.
    dividend_adjustment: Option<AccountUnits>,
    /// The dividend adjustment amount to pay or collect for the
    /// Trade, in the Instrument’s quote currency.
    quote_dividend_adjustment: Option<DecimalNumber>,
}
impl Default for OpenTradeDividendAdjustment {
    fn default() -> Self {
        Self {
            trade_id: Default::default(),
            dividend_adjustment: Default::default(),
            quote_dividend_adjustment: Default::default(),
        }
    }
}
