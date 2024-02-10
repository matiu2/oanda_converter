use serde_inline_default::serde_inline_default;
use crate::definitions::time_in_force::TimeInForce;
use crate::definitions::order_trigger_condition::OrderTriggerCondition;
use crate::definitions::client_id::ClientID;
use crate::definitions::order_type::OrderType;
use chrono::DateTime;
use crate::definitions::decimal_number::DecimalNumber;
use chrono::Utc;
use crate::definitions::price_value::PriceValue;
use crate::endpoints::trade::responses::client_extensions::ClientExtensions;
use crate::definitions::trade_id::TradeID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct GuaranteedStopLossOrderRequest {
    /// The type of the Order to Create. Must be set to
    /// “GUARANTEED_STOP_LOSS” when creating a Guaranteed Stop Loss
    /// Order.
    #[serde_inline_default("GUARANTEED_STOP_LOSS")]
    r#type: OrderType,
    /// The ID of the Trade to close when the price threshold is
    /// breached.
    trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price
    /// threshold is breached.
    client_trade_id: Option<ClientID>,
    /// The price threshold specified for the Guaranteed Stop Loss
    /// Order. The associated Trade will be closed at this price.
    price: PriceValue,
    /// Specifies the distance (in price units) from the Account’s
    /// current price to use as the Guaranteed Stop Loss Order
    /// price. If the Trade is short the Instrument’s bid price is
    /// used, and for long Trades the ask is used.
    distance: Option<DecimalNumber>,
    /// The time-in-force requested for the GuaranteedStopLoss
    /// Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// GuaranteedStopLoss Orders.
    #[serde_inline_default("GTC")]
    time_in_force: TimeInForce,
    /// The date/time when the GuaranteedStopLoss Order will be
    /// cancelled if its timeInForce is “GTD”.
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
impl Default for GuaranteedStopLossOrderRequest {
    fn default() -> Self {
        Self {
            r#type: "GUARANTEED_STOP_LOSS",
            trade_id: Default::default(),
            client_trade_id: Default::default(),
            price: Default::default(),
            distance: Default::default(),
            time_in_force: "GTC",
            gtd_time: Default::default(),
            trigger_condition: "DEFAULT",
            client_extensions: Default::default(),
        }
    }
}
