use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct TradeReduce {
    /// The ID of the Trade that was reduced or closed
    tradeID: Option<TradeID>,
    /// The number of units that the Trade was reduced by
    units: Option<DecimalNumber>,
    /// The average price that the units were closed at. This price may be clamped for guaranteed Stop Loss Orders.
    price: Option<PriceValue>,
    /// The PL realized when reducing the Trade
    realizedPL: Option<AccountUnits>,
    /// The financing paid/collected when reducing the Trade
    financing: Option<AccountUnits>,
    /// The base financing paid/collected when reducing the Trade
    baseFinancing: Option<DecimalNumber>,
    /// The quote financing paid/collected when reducing the Trade
    quoteFinancing: Option<DecimalNumber>,
    /// The financing rate in effect for the instrument used to calculate the amount of financing paid/collected when reducing the Trade. This field will only be set if the AccountFinancingMode at the time of the order fill is SECOND_BY_SECOND_INSTRUMENT. The value is in decimal rather than percentage points, e.g. 5% is represented as 0.05.
    financingRate: Option<DecimalNumber>,
    /// This is the fee that is charged for closing the Trade if it has a guaranteed Stop Loss Order attached to it.
    guaranteedExecutionFee: Option<AccountUnits>,
    /// This is the fee that is charged for closing the Trade if it has a guaranteed Stop Loss Order attached to it, expressed in the Instrument’s quote currency.
    quoteGuaranteedExecutionFee: Option<DecimalNumber>,
    /// The half spread cost for the trade reduce/close. This can be a positive or negative value and is represented in the home currency of the Account.
    halfSpreadCost: Option<AccountUnits>,
}
