use definitions::transaction_id::TransactionID;
use chrono::DateTime;
use definitions::trailing_stop_loss_order_reason::TrailingStopLossOrderReason;
use definitions::order_id::OrderID;
use definitions::trade_id::TradeID;
use definitions::client_extensions::ClientExtensions;
use definitions::time_in_force::TimeInForce;
use definitions::client_id::ClientID;
use definitions::account_id::AccountID;
use definitions::order_trigger_condition::OrderTriggerCondition;
use definitions::decimal_number::DecimalNumber;
use definitions::request_id::RequestID;
use definitions::transaction_type::TransactionType;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TrailingStopLossOrderTransaction {
    /// The Transaction’s Identifier.
    id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    time: Option<DateTime>,
    /// The ID of the user that initiated the creation of the
    /// Transaction.
    user_id: Option<integer>,
    /// The ID of the Account the Transaction was created for.
    account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to.
    /// Transactions in the same batch are applied to the Account
    /// simultaneously.
    batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the
    /// transaction.
    request_id: Option<RequestID>,
    /// The Type of the Transaction. Always
    /// set to “TRAILING_STOP_LOSS_ORDER” in a
    /// TrailingStopLossOrderTransaction.
    #[serde_inline_default("TRAILING_STOP_LOSS_ORDER")]
    r#type: TransactionType,
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
    /// The reason that the Trailing Stop Loss Order was initiated
    reason: Option<TrailingStopLossOrderReason>,
    /// Client Extensions to add to the Order (only provided if the
    /// Order is being created with client extensions).
    client_extensions: Option<ClientExtensions>,
    /// The ID of the OrderFill Transaction that caused this Order
    /// to be created (only provided if this Order was created
    /// automatically when another Order was filled).
    order_fill_transaction_id: Option<TransactionID>,
    /// The ID of the Order that this Order replaces (only provided
    /// if this Order replaces an existing Order).
    replaces_order_id: Option<OrderID>,
    /// The ID of the Transaction that cancels the replaced Order
    /// (only provided if this Order replaces an existing Order).
    cancelling_transaction_id: Option<TransactionID>,
}
impl Default for TrailingStopLossOrderTransaction {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            time: default(),
            user_id: default(),
            account_id: default(),
            batch_id: default(),
            request_id: default(),
            r#type: "TRAILING_STOP_LOSS_ORDER",
            trade_id: default(),
            client_trade_id: default(),
            distance: default(),
            time_in_force: "GTC",
            gtd_time: default(),
            trigger_condition: "DEFAULT",
            reason: default(),
            client_extensions: default(),
            order_fill_transaction_id: default(),
            replaces_order_id: default(),
            cancelling_transaction_id: default(),
        }
    }
}
