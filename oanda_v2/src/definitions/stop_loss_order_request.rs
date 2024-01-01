use definitions::order_type::OrderType;
use chrono::DateTime;
use definitions::order_trigger_condition::OrderTriggerCondition;
use definitions::time_in_force::TimeInForce;
use definitions::price_value::PriceValue;
use definitions::trade_id::TradeID;
use definitions::client_extensions::ClientExtensions;
use definitions::client_id::ClientID;
use definitions::decimal_number::DecimalNumber;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct StopLossOrderRequest {
    /// The type of the Order to Create. Must be set to “STOP_LOSS”
    /// when creating a Stop Loss Order.
    #[serde_inline_default("STOP_LOSS")]
    r#type: OrderType,
    /// The ID of the Trade to close when the price threshold is
    /// breached.
    trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price
    /// threshold is breached.
    client_trade_id: Option<ClientID>,
    /// The price threshold specified for the Stop Loss Order. The
    /// associated Trade will be closed by a market price that is
    /// equal to or worse than this threshold.
    price: PriceValue,
    /// Specifies the distance (in price units) from the Account’s
    /// current price to use as the Stop Loss Order price. If the
    /// Trade is short the Instrument’s bid price is used, and for
    /// long Trades the ask is used.
    distance: Option<DecimalNumber>,
    /// The time-in-force requested for the StopLoss Order.
    /// Restricted to “GTC”, “GFD” and “GTD” for StopLoss Orders.
    #[serde_inline_default("GTC")]
    time_in_force: TimeInForce,
    /// The date/time when the StopLoss Order will be cancelled if
    /// its timeInForce is “GTD”.
    gtd_time: Option<DateTime>,
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
impl Default for StopLossOrderRequest {
    fn default() -> Self {
        use Default::default;
        Self {
            r#type: "STOP_LOSS",
            trade_id: default(),
            client_trade_id: default(),
            price: default(),
            distance: default(),
            time_in_force: "GTC",
            gtd_time: default(),
            trigger_condition: "DEFAULT",
            client_extensions: default(),
        }
    }
}
