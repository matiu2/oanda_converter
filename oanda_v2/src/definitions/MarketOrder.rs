use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct MarketOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    id: Option<OrderID>,
    /// The time when the Order was created.
    create_time: Option<DateTime>,
    /// The current state of the Order.
    state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is associated with MT4.
    client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “MARKET” for Market Orders.
    #[serde(default = "MARKET")]
    r#type: OrderType,
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
    /// TakeProfitDetails specifies the details of a Take Profit Order to be created on behalf of a client. This may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s dependent Take Profit Order is modified directly through the Trade.
    take_profit_on_fill: Option<TakeProfitDetails>,
    /// StopLossDetails specifies the details of a Stop Loss Order to be created on behalf of a client. This may happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent Stop Loss Order is modified directly through the Trade.
    stop_loss_on_fill: Option<StopLossDetails>,
    /// GuaranteedStopLossDetails specifies the details of a Guaranteed Stop Loss Order to be created on behalf of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
    guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// TrailingStopLossDetails specifies the details of a Trailing Stop Loss Order to be created on behalf of a client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
    trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do not set, modify, or delete tradeClientExtensions if your account is associated with MT4.
    trade_client_extensions: Option<ClientExtensions>,
    /// ID of the Transaction that filled this Order (only provided when the Order’s state is FILLED)
    filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the Order’s state is FILLED)
    filled_time: Option<DateTime>,
    /// Trade ID of Trade opened when the Order was filled (only provided when the Order’s state is FILLED and a Trade was opened as a result of the fill)
    trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only provided when the Order’s state is FILLED and a Trade was reduced as a result of the fill)
    trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only provided when the Order’s state is FILLED and one or more Trades were closed as a result of the fill)
    trade_closed_i_ds: Vec<TradeID>,
    /// ID of the Transaction that cancelled the Order (only provided when the Order’s state is CANCELLED)
    cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when the state of the Order is CANCELLED)
    cancelled_time: Option<DateTime>,
}
