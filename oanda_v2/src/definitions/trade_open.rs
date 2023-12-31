use definitions::price_value::PriceValue;
use definitions::decimal_number::DecimalNumber;
use definitions::account_units::AccountUnits;
use definitions::client_extensions::ClientExtensions;
use definitions::trade_id::TradeID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TradeOpen {
    /// The ID of the Trade that was opened
    trade_id: Option<TradeID>,
    /// The number of units opened by the Trade
    units: Option<DecimalNumber>,
    /// The average price that the units were opened at.
    price: Option<PriceValue>,
    /// This is the fee charged for opening the trade if it has a
    /// guaranteed Stop Loss Order attached to it.
    guaranteed_execution_fee: Option<AccountUnits>,
    /// This is the fee charged for opening the trade if it has a
    /// guaranteed Stop Loss Order attached to it, expressed in the
    /// Instrument’s quote currency.
    quote_guaranteed_execution_fee: Option<DecimalNumber>,
    /// The client extensions for the newly opened Trade
    client_extensions: Option<ClientExtensions>,
    /// The half spread cost for the trade open. This can be a
    /// positive or negative value and is represented in the home
    /// currency of the Account.
    half_spread_cost: Option<AccountUnits>,
    /// The margin required at the time the Trade was created.
    /// Note, this is the ‘pure’ margin required, it is not the
    /// ‘effective’ margin used that factors in the trade risk if a
    /// GSLO is attached to the trade.
    initial_margin_required: Option<AccountUnits>,
}
