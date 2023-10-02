use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct OpenTradeDividendAdjustment {
    /// The ID of the Trade for which the dividend adjustment is to be paid or collected.
    tradeID: Option<TradeID>,
    /// The dividend adjustment amount to pay or collect for the Trade.
    dividendAdjustment: Option<AccountUnits>,
    /// The dividend adjustment amount to pay or collect for the Trade, in the Instrument’s quote currency.
    quoteDividendAdjustment: Option<DecimalNumber>,
}
