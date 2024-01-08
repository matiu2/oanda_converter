use definitions::open_trade_financing::OpenTradeFinancing;
use definitions::account_units::AccountUnits;
use definitions::instrument_name::InstrumentName;
use definitions::decimal_number::DecimalNumber;
use definitions::account_financing_mode::AccountFinancingMode;
use definitions::home_conversion_factors::HomeConversionFactors;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct PositionFinancing {
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
impl Default for PositionFinancing {
    fn default() -> Self {
        use Default::default;
        Self {
            instrument: default(),
            financing: default(),
            base_financing: default(),
            quote_financing: default(),
            home_conversion_factors: default(),
            open_trade_financings: default(),
            account_financing_mode: default(),
        }
    }
}
