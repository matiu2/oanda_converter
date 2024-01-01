use definitions::account_units::AccountUnits;
use definitions::decimal_number::DecimalNumber;
use definitions::trade_id::TradeID;
use definitions::price_value::PriceValue;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TradeReduce {
    /// The ID of the Trade that was reduced or closed
    trade_id: Option<TradeID>,
    /// The number of units that the Trade was reduced by
    units: Option<DecimalNumber>,
    /// The average price that the units were closed at. This price
    /// may be clamped for guaranteed Stop Loss Orders.
    price: Option<PriceValue>,
    /// The PL realized when reducing the Trade
    realized_pl: Option<AccountUnits>,
    /// The financing paid/collected when reducing the Trade
    financing: Option<AccountUnits>,
    /// The base financing paid/collected when reducing the Trade
    base_financing: Option<DecimalNumber>,
    /// The quote financing paid/collected when reducing the Trade
    quote_financing: Option<DecimalNumber>,
    /// The financing rate in effect for the instrument used to
    /// calculate the amount of financing paid/collected when
    /// reducing the Trade. This field will only be set if the
    /// AccountFinancingMode at the time of the order fill is
    /// SECOND_BY_SECOND_INSTRUMENT. The value is in decimal rather
    /// than percentage points, e.g. 5% is represented as 0.05.
    financing_rate: Option<DecimalNumber>,
    /// This is the fee that is charged for closing the Trade if it
    /// has a guaranteed Stop Loss Order attached to it.
    guaranteed_execution_fee: Option<AccountUnits>,
    /// This is the fee that is charged for closing the Trade if it
    /// has a guaranteed Stop Loss Order attached to it, expressed
    /// in the Instrument’s quote currency.
    quote_guaranteed_execution_fee: Option<DecimalNumber>,
    /// The half spread cost for the trade reduce/close. This can be
    /// a positive or negative value and is represented in the home
    /// currency of the Account.
    half_spread_cost: Option<AccountUnits>,
}
impl Default for TradeReduce {
    fn default() -> Self {
        use Default::default;
        Self {
            trade_id: default(),
            units: default(),
            price: default(),
            realized_pl: default(),
            financing: default(),
            base_financing: default(),
            quote_financing: default(),
            financing_rate: default(),
            guaranteed_execution_fee: default(),
            quote_guaranteed_execution_fee: default(),
            half_spread_cost: default(),
        }
    }
}
