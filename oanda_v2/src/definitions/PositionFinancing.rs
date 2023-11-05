use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct PositionFinancing {
    /// The instrument of the Position that financing is being paid/
    /// collected for.
    instrument: Option<InstrumentName>,
    /// The amount of financing paid/collected for the Position.
    financing: Option<AccountUnits>,
    /// The amount of base financing paid/collected for the
    /// Position.
    base_financing: Option<DecimalNumber>,
    /// The amount of quote financing paid/collected for the
    /// Position.
    quote_financing: Option<DecimalNumber>,
    /// The HomeConversionFactors in effect for the Position’s
    /// Instrument at the time of the DailyFinancing.
    home_conversion_factors: Option<HomeConversionFactors>,
    /// The financing paid/collected for each open Trade within
    /// the Position.
    open_trade_financings: Vec<OpenTradeFinancing>,
    /// The account financing mode at the time of the daily
    /// financing.
    account_financing_mode: Option<AccountFinancingMode>,
}
