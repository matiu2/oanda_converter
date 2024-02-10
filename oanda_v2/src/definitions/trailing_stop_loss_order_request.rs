use crate::definitions::client_id::ClientID;
use crate::definitions::trade_id::TradeID;
use crate::endpoints::trade::responses::client_extensions::ClientExtensions;
use crate::definitions::order_trigger_condition::OrderTriggerCondition;
use crate::definitions::decimal_number::DecimalNumber;
use chrono::DateTime;
use chrono::Utc;
use crate::definitions::time_in_force::TimeInForce;
use crate::definitions::order_type::OrderType;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TrailingStopLossOrderRequest {
    /// The type of the Order to Create. Must be set to
    /// “TRAILING_STOP_LOSS” when creating a Trailing Stop Loss
    /// Order.
    #[serde_inline_default("TRAILING_STOP_LOSS")]
    r#type: OrderType,
    /// The ID of the Trade to close when the price threshold is
    /// breached.
    trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price
    /// threshold is breached.
    client_trade_id: Option<ClientID>,
    /// The price distance (in price units) specified for the
    /// TrailingStopLoss Order.
    distance: DecimalNumber,
    /// The time-in-force requested for the TrailingStopLoss Order.
    /// Restricted to “GTC”, “GFD” and “GTD” for TrailingStopLoss
    /// Orders.
    #[serde_inline_default("GTC")]
    time_in_force: TimeInForce,
    /// The date/time when the StopLoss Order will be cancelled if
    /// its timeInForce is “GTD”.
    gtd_time: Option<DateTime<Utc>>,
    /// Specification of which price component should be used when
    /// determining if an Order should be triggered and filled.
    /// This allows Orders to be triggered based on the bid, ask,
    /// mid, default (ask for buy, bid for sell) or inverse (ask for
    /// sell, bid for buy) price depending on the desired behaviour.
    /// Orders are always filled using their default price
    /// component. This feature is only provided through the REST
    /// API. Clients who choose to specify a non-default trigger
    /// condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction
    /// history or their account statements. OANDA platforms always
    /// assume that an Order’s trigger condition is set to the
    /// default value when indicating the distance from an Order’s
    /// trigger price, and will always provide the default trigger
    /// condition when creating or modifying an Order. A special
    /// restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either
    /// be “DEFAULT”, or the “natural” trigger side “DEFAULT”
    /// results in. So for a Guaranteed Stop Loss Order for a long
    /// trade valid values are “DEFAULT” and “BID”, and for short
    /// trades “DEFAULT” and “ASK” are valid.
    #[serde_inline_default("DEFAULT")]
    trigger_condition: OrderTriggerCondition,
    /// The client extensions to add to the Order. Do not set,
    /// modify, or delete clientExtensions if your account is
    /// associated with MT4.
    client_extensions: Option<ClientExtensions>,
}
impl Default for TrailingStopLossOrderRequest {
    fn default() -> Self {
        Self {
            r#type: "TRAILING_STOP_LOSS",
            trade_id: Default::default(),
            client_trade_id: Default::default(),
            distance: Default::default(),
            time_in_force: "GTC",
            gtd_time: Default::default(),
            trigger_condition: "DEFAULT",
            client_extensions: Default::default(),
        }
    }
}
