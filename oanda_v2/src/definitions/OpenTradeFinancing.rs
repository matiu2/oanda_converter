use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct OpenTradeFinancing {
    /// The ID of the Trade that financing is being paid/collected for.
    tradeID: Option<TradeID>,
    /// The amount of financing paid/collected for the Trade.
    financing: Option<AccountUnits>,
    /// The amount of financing paid/collected in the Instrument’s base currency for the Trade.
    baseFinancing: Option<DecimalNumber>,
    /// The amount of financing paid/collected in the Instrument’s quote currency for the Trade.
    quoteFinancing: Option<DecimalNumber>,
    /// The financing rate in effect for the instrument used to calculate the the amount of financing paid/collected for the Trade. This field will only be set if the AccountFinancingMode at the time of the daily financing is DAILY_INSTRUMENT or SECOND_BY_SECOND_INSTRUMENT. The value is in decimal rather than percentage points, e.g. 5% is represented as 0.05.
    financingRate: Option<DecimalNumber>,
}
