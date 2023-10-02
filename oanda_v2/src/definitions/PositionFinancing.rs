use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct PositionFinancing {
    /// The instrument of the Position that financing is being paid/collected for.
    instrument: Option<InstrumentName>,
    /// The amount of financing paid/collected for the Position.
    financing: Option<AccountUnits>,
    /// The amount of base financing paid/collected for the Position.
    baseFinancing: Option<DecimalNumber>,
    /// The amount of quote financing paid/collected for the Position.
    quoteFinancing: Option<DecimalNumber>,
    /// The HomeConversionFactors in effect for the Position’s Instrument at the time of the DailyFinancing.
    homeConversionFactors: Option<HomeConversionFactors>,
    /// The financing paid/collected for each open Trade within the Position.
    openTradeFinancings: Vec<OpenTradeFinancing>,
    /// The account financing mode at the time of the daily financing.
    accountFinancingMode: Option<AccountFinancingMode>,
}
