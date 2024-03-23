use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct TakeProfitOrderRejectTransaction {
    /// The Transaction’s Identifier.
    id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    time: Option<DateTime<Utc>>,
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
    /// set to “TAKE_PROFIT_ORDER_REJECT” in a
    /// TakeProfitOrderRejectTransaction.
    #[serde_inline_default("TAKE_PROFIT_ORDER_REJECT")]
    r#type: TransactionType,
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
    /// The reason that the Take Profit Order was initiated
    reason: Option<TakeProfitOrderReason>,
    /// Client Extensions to add to the Order (only provided if the
    /// Order is being created with client extensions).
    client_extensions: Option<ClientExtensions>,
    /// The ID of the OrderFill Transaction that caused this Order
    /// to be created (only provided if this Order was created
    /// automatically when another Order was filled).
    order_fill_transaction_id: Option<TransactionID>,
    /// The ID of the Order that this Order was intended to replace
    /// (only provided if this Order was intended to replace an
    /// existing Order).
    intended_replaces_order_id: Option<OrderID>,
    /// The reason that the Reject Transaction was created
    reject_reason: Option<TransactionRejectReason>,
}
impl Default for TakeProfitOrderRejectTransaction {
    fn default() -> Self {
        Self {
            id: Default::default(),
            time: Default::default(),
            user_id: Default::default(),
            account_id: Default::default(),
            batch_id: Default::default(),
            request_id: Default::default(),
            r#type: "TAKE_PROFIT_ORDER_REJECT",
            trade_id: Default::default(),
            client_trade_id: Default::default(),
            price: Default::default(),
            time_in_force: "GTC",
            gtd_time: Default::default(),
            trigger_condition: "DEFAULT",
            reason: Default::default(),
            client_extensions: Default::default(),
            order_fill_transaction_id: Default::default(),
            intended_replaces_order_id: Default::default(),
            reject_reason: Default::default(),
        }
    }
}
