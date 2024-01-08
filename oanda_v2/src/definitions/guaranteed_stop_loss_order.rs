use crate::definitions::order_trigger_condition::OrderTriggerCondition;
use crate::definitions::trade_id::TradeID;
use crate::definitions::price_value::PriceValue;
use crate::definitions::order_state::OrderState;
use crate::definitions::time_in_force::TimeInForce;
use crate::definitions::decimal_number::DecimalNumber;
use crate::definitions::order_type::OrderType;
use crate::definitions::transaction_id::TransactionID;
use crate::definitions::client_extensions::ClientExtensions;
use crate::definitions::order_id::OrderID;
use crate::chrono::DateTime;
use crate::definitions::client_id::ClientID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct GuaranteedStopLossOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    id: Option<OrderID>,
    /// The time when the Order was created.
    create_time: Option<DateTime>,
    /// The current state of the Order.
    state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or
    /// delete clientExtensions if your account is associated with
    /// MT4.
    client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “GUARANTEED_STOP_LOSS”
    /// for Guaranteed Stop Loss Orders.
    #[serde_inline_default("GUARANTEED_STOP_LOSS")]
    r#type: OrderType,
    /// The premium that will be charged if the Guaranteed Stop
    /// Loss Order is filled at the guaranteed price. It is in price
    /// units and is charged for each unit of the Trade.
    guaranteed_execution_premium: Option<DecimalNumber>,
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
    /// ID of the Transaction that filled this Order (only provided
    /// when the Order’s state is FILLED)
    filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the
    /// Order’s state is FILLED)
    filled_time: Option<DateTime>,
    /// Trade ID of Trade opened when the Order was filled (only
    /// provided when the Order’s state is FILLED and a Trade was
    /// opened as a result of the fill)
    trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only
    /// provided when the Order’s state is FILLED and a Trade was
    /// reduced as a result of the fill)
    trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only
    /// provided when the Order’s state is FILLED and one or more
    /// Trades were closed as a result of the fill)
    trade_closed_i_ds: Vec<TradeID>,
    /// ID of the Transaction that cancelled the Order (only
    /// provided when the Order’s state is CANCELLED)
    cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when
    /// the state of the Order is CANCELLED)
    cancelled_time: Option<DateTime>,
    /// The ID of the Order that was replaced by this Order (only
    /// provided if this Order was created as part of a cancel/
    /// replace).
    replaces_order_id: Option<OrderID>,
    /// The ID of the Order that replaced this Order (only provided
    /// if this Order was cancelled as part of a cancel/replace).
    replaced_by_order_id: Option<OrderID>,
}
impl Default for GuaranteedStopLossOrder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            create_time: Default::default(),
            state: Default::default(),
            client_extensions: Default::default(),
            r#type: "GUARANTEED_STOP_LOSS",
            guaranteed_execution_premium: Default::default(),
            trade_id: Default::default(),
            client_trade_id: Default::default(),
            price: Default::default(),
            distance: Default::default(),
            time_in_force: "GTC",
            gtd_time: Default::default(),
            trigger_condition: "DEFAULT",
            filling_transaction_id: Default::default(),
            filled_time: Default::default(),
            trade_opened_id: Default::default(),
            trade_reduced_id: Default::default(),
            trade_closed_i_ds: Default::default(),
            cancelling_transaction_id: Default::default(),
            cancelled_time: Default::default(),
            replaces_order_id: Default::default(),
            replaced_by_order_id: Default::default(),
        }
    }
}
