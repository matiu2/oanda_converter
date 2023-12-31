use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct LimitOrderTransaction {
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
    /// The Type of the Transaction. Always set to “LIMIT_ORDER” in
    /// a LimitOrderTransaction.
    #[serde(default = "LIMIT_ORDER")]
    r#type: TransactionType,
    /// The Limit Order’s Instrument.
    instrument: InstrumentName,
    /// The quantity requested to be filled by the Limit Order.
    /// A positive number of units results in a long Order, and a
    /// negative number of units results in a short Order.
    units: DecimalNumber,
    /// The price threshold specified for the Limit Order. The Limit
    /// Order will only be filled by a market price that is equal to
    /// or better than this price.
    price: PriceValue,
    /// The time-in-force requested for the Limit Order.
    #[serde(default = "GTC")]
    time_in_force: TimeInForce,
    /// The date/time when the Limit Order will be cancelled if its
    /// timeInForce is “GTD”.
    gtd_time: Option<DateTime>,
    /// Specification of how Positions in the Account are modified
    /// when the Order is filled.
    #[serde(default = "DEFAULT")]
    position_fill: OrderPositionFill,
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
    #[serde(default = "DEFAULT")]
    trigger_condition: OrderTriggerCondition,
    /// The reason that the Limit Order was initiated
    reason: Option<LimitOrderReason>,
    /// Client Extensions to add to the Order (only provided if the
    /// Order is being created with client extensions).
    client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be
    /// created for a Trade opened when the Order is filled (if such
    /// a Trade is created).
    take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be
    /// created for a Trade opened when the Order is filled (if such
    /// a Trade is created).
    stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that
    /// should be created for a Trade that is opened when the Order
    /// is filled (if such a Trade is created).
    trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that
    /// should be created for a Trade that is opened when the Order
    /// is filled (if such a Trade is created).
    guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order
    /// is filled (if such a Trade is created).  Do not set, modify,
    /// delete tradeClientExtensions if your account is associated
    /// with MT4.
    trade_client_extensions: Option<ClientExtensions>,
    /// The ID of the Order that this Order replaces (only provided
    /// if this Order replaces an existing Order).
    replaces_order_id: Option<OrderID>,
    /// The ID of the Transaction that cancels the replaced Order
    /// (only provided if this Order replaces an existing Order).
    cancelling_transaction_id: Option<TransactionID>,
}
