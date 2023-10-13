use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct MarketOrderTransaction {
    /// The Transaction’s Identifier.
    id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    time: Option<DateTime>,
    /// The ID of the user that initiated the creation of the Transaction.
    user_id: Option<integer>,
    /// The ID of the Account the Transaction was created for.
    account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the Account simultaneously.
    batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “MARKET_ORDER” in a MarketOrderTransaction.
    #[serde(default = "MARKET_ORDER")]
    r#type: TransactionType,
    /// The Market Order’s Instrument.
    instrument: InstrumentName,
    /// The quantity requested to be filled by the Market Order. A positive number of units results in a long Order, and a negative number of units results in a short Order.
    units: DecimalNumber,
    /// The time-in-force requested for the Market Order. Restricted to FOK or IOC for a MarketOrder.
    #[serde(default = "FOK")]
    time_in_force: TimeInForce,
    /// The worst price that the client is willing to have the Market Order filled at.
    price_bound: Option<PriceValue>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(default = "DEFAULT")]
    position_fill: OrderPositionFill,
    /// Details of the Trade requested to be closed, only provided when the Market Order is being used to explicitly close a Trade.
    trade_close: Option<MarketOrderTradeClose>,
    /// Details of the long Position requested to be closed out, only provided when a Market Order is being used to explicitly closeout a long Position.
    long_position_closeout: Option<MarketOrderPositionCloseout>,
    /// Details of the short Position requested to be closed out, only provided when a Market Order is being used to explicitly closeout a short Position.
    short_position_closeout: Option<MarketOrderPositionCloseout>,
    /// Details of the Margin Closeout that this Market Order was created for
    margin_closeout: Option<MarketOrderMarginCloseout>,
    /// Details of the delayed Trade close that this Market Order was created for
    delayed_trade_close: Option<MarketOrderDelayedTradeClose>,
    /// The reason that the Market Order was created
    reason: Option<MarketOrderReason>,
    /// Client Extensions to add to the Order (only provided if the Order is being created with client extensions).
    client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be created for a Trade opened when the Order is filled (if such a Trade is created).
    take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be created for a Trade opened when the Order is filled (if such a Trade is created).
    stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when the Order is filled (if such a Trade is created).
    trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened when the Order is filled (if such a Trade is created).
    guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created).  Do not set, modify, delete tradeClientExtensions if your account is associated with MT4.
    trade_client_extensions: Option<ClientExtensions>,
}
