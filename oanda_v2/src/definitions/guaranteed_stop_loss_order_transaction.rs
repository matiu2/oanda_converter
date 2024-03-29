use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct GuaranteedStopLossOrderTransaction {
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
    /// The Type of the Transaction. Always set
    /// to “GUARANTEED_STOP_LOSS_ORDER” in a
    /// GuaranteedStopLossOrderTransaction.
    #[serde_inline_default("GUARANTEED_STOP_LOSS_ORDER")]
    r#type: TransactionType,
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
    /// The fee that will be charged if the Guaranteed Stop Loss
    /// Order is filled at the guaranteed price. The value is
    /// determined at Order creation time. It is in price units and
    /// is charged for each unit of the Trade.
    guaranteed_execution_premium: Option<DecimalNumber>,
    /// The reason that the Guaranteed Stop Loss Order was initiated
    reason: Option<GuaranteedStopLossOrderReason>,
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
impl Default for GuaranteedStopLossOrderTransaction {
    fn default() -> Self {
        Self {
            id: Default::default(),
            time: Default::default(),
            user_id: Default::default(),
            account_id: Default::default(),
            batch_id: Default::default(),
            request_id: Default::default(),
            r#type: "GUARANTEED_STOP_LOSS_ORDER",
            trade_id: Default::default(),
            client_trade_id: Default::default(),
            price: Default::default(),
            distance: Default::default(),
            time_in_force: "GTC",
            gtd_time: Default::default(),
            trigger_condition: "DEFAULT",
            guaranteed_execution_premium: Default::default(),
            reason: Default::default(),
            client_extensions: Default::default(),
            order_fill_transaction_id: Default::default(),
            replaces_order_id: Default::default(),
            cancelling_transaction_id: Default::default(),
        }
    }
}
