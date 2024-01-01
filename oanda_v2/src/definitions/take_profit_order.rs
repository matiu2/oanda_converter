use definitions::price_value::PriceValue;
use definitions::time_in_force::TimeInForce;
use definitions::client_id::ClientID;
use definitions::transaction_id::TransactionID;
use definitions::client_extensions::ClientExtensions;
use definitions::order_id::OrderID;
use chrono::DateTime;
use definitions::trade_id::TradeID;
use definitions::order_trigger_condition::OrderTriggerCondition;
use definitions::order_type::OrderType;
use definitions::order_state::OrderState;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TakeProfitOrder {
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
    /// The type of the Order. Always set to “TAKE_PROFIT” for Take
    /// Profit Orders.
    #[serde_inline_default("TAKE_PROFIT")]
    r#type: OrderType,
    /// The ID of the Trade to close when the price threshold is
    /// breached.
    trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price
    /// threshold is breached.
    client_trade_id: Option<ClientID>,
    /// The price threshold specified for the TakeProfit Order. The
    /// associated Trade will be closed by a market price that is
    /// equal to or better than this threshold.
    price: PriceValue,
    /// The time-in-force requested for the TakeProfit Order.
    /// Restricted to “GTC”, “GFD” and “GTD” for TakeProfit Orders.
    #[serde_inline_default("GTC")]
    time_in_force: TimeInForce,
    /// The date/time when the TakeProfit Order will be cancelled if
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
impl Default for TakeProfitOrder {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            create_time: default(),
            state: default(),
            client_extensions: default(),
            r#type: "TAKE_PROFIT",
            trade_id: default(),
            client_trade_id: default(),
            price: default(),
            time_in_force: "GTC",
            gtd_time: default(),
            trigger_condition: "DEFAULT",
            filling_transaction_id: default(),
            filled_time: default(),
            trade_opened_id: default(),
            trade_reduced_id: default(),
            trade_closed_i_ds: default(),
            cancelling_transaction_id: default(),
            cancelled_time: default(),
            replaces_order_id: default(),
            replaced_by_order_id: default(),
        }
    }
}
