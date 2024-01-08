use crate::definitions::decimal_number::DecimalNumber;
use crate::definitions::instrument_name::InstrumentName;
use crate::definitions::price_value::PriceValue;
use crate::definitions::time_in_force::TimeInForce;
use crate::definitions::order_position_fill::OrderPositionFill;
use crate::definitions::order_type::OrderType;
use crate::definitions::trailing_stop_loss_details::TrailingStopLossDetails;
use crate::definitions::guaranteed_stop_loss_details::GuaranteedStopLossDetails;
use crate::definitions::client_extensions::ClientExtensions;
use crate::definitions::take_profit_details::TakeProfitDetails;
use crate::definitions::stop_loss_details::StopLossDetails;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct MarketOrderRequest {
    /// The type of the Order to Create. Must be set to “MARKET”
    /// when creating a Market Order.
    #[serde_inline_default("MARKET")]
    r#type: OrderType,
    /// The Market Order’s Instrument.
    instrument: InstrumentName,
    /// The quantity requested to be filled by the Market Order.
    /// A positive number of units results in a long Order, and a
    /// negative number of units results in a short Order.
    units: DecimalNumber,
    /// The time-in-force requested for the Market Order. Restricted
    /// to FOK or IOC for a MarketOrder.
    #[serde_inline_default("FOK")]
    time_in_force: TimeInForce,
    /// The worst price that the client is willing to have the
    /// Market Order filled at.
    price_bound: Option<PriceValue>,
    /// Specification of how Positions in the Account are modified
    /// when the Order is filled.
    #[serde_inline_default("DEFAULT")]
    position_fill: OrderPositionFill,
    /// The client extensions to add to the Order. Do not set,
    /// modify, or delete clientExtensions if your account is
    /// associated with MT4.
    client_extensions: Option<ClientExtensions>,
    /// TakeProfitDetails specifies the details of a Take Profit
    /// Order to be created on behalf of a client. This may happen
    /// when an Order is filled that opens a Trade requiring a Take
    /// Profit, or when a Trade’s dependent Take Profit Order is
    /// modified directly through the Trade.
    take_profit_on_fill: Option<TakeProfitDetails>,
    /// StopLossDetails specifies the details of a Stop Loss Order
    /// to be created on behalf of a client. This may happen when
    /// an Order is filled that opens a Trade requiring a Stop Loss,
    /// or when a Trade’s dependent Stop Loss Order is modified
    /// directly through the Trade.
    stop_loss_on_fill: Option<StopLossDetails>,
    /// GuaranteedStopLossDetails specifies the details of a
    /// Guaranteed Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens
    /// a Trade requiring a Guaranteed Stop Loss, or when a Trade’s
    /// dependent Guaranteed Stop Loss Order is modified directly
    /// through the Trade.
    guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// TrailingStopLossDetails specifies the details of a Trailing
    /// Stop Loss Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade
    /// requiring a Trailing Stop Loss, or when a Trade’s dependent
    /// Trailing Stop Loss Order is modified directly through the
    /// Trade.
    trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the
    /// Order is filled (if such a Trade is created). Do not set,
    /// modify, or delete tradeClientExtensions if your account is
    /// associated with MT4.
    trade_client_extensions: Option<ClientExtensions>,
}
impl Default for MarketOrderRequest {
    fn default() -> Self {
        Self {
            r#type: "MARKET",
            instrument: Default::default(),
            units: Default::default(),
            time_in_force: "FOK",
            price_bound: Default::default(),
            position_fill: "DEFAULT",
            client_extensions: Default::default(),
            take_profit_on_fill: Default::default(),
            stop_loss_on_fill: Default::default(),
            guaranteed_stop_loss_on_fill: Default::default(),
            trailing_stop_loss_on_fill: Default::default(),
            trade_client_extensions: Default::default(),
        }
    }
}
