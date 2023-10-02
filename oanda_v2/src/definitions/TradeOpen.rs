use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct TradeOpen {
    /// The ID of the Trade that was opened
    tradeID: Option<TradeID>,
    /// The number of units opened by the Trade
    units: Option<DecimalNumber>,
    /// The average price that the units were opened at.
    price: Option<PriceValue>,
    /// This is the fee charged for opening the trade if it has a guaranteed Stop Loss Order attached to it.
    guaranteedExecutionFee: Option<AccountUnits>,
    /// This is the fee charged for opening the trade if it has a guaranteed Stop Loss Order attached to it, expressed in the Instrument’s quote currency.
    quoteGuaranteedExecutionFee: Option<DecimalNumber>,
    /// The client extensions for the newly opened Trade
    clientExtensions: Option<ClientExtensions>,
    /// The half spread cost for the trade open. This can be a positive or negative value and is represented in the home currency of the Account.
    halfSpreadCost: Option<AccountUnits>,
    /// The margin required at the time the Trade was created. Note, this is the ‘pure’ margin required, it is not the ‘effective’ margin used that factors in the trade risk if a GSLO is attached to the trade.
    initialMarginRequired: Option<AccountUnits>,
}
