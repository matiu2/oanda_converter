use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct StopOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    id: Option<OrderID>,
    /// The time when the Order was created.
    create_time: Option<DateTime<Utc>>,
    /// The current state of the Order.
    state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or
    /// delete clientExtensions if your account is associated with
    /// MT4.
    client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “STOP” for Stop Orders.
    #[serde_inline_default("STOP")]
    r#type: OrderType,
    /// The Stop Order’s Instrument.
    instrument: InstrumentName,
    /// The quantity requested to be filled by the Stop Order. A
    /// positive number of units results in a long Order, and a
    /// negative number of units results in a short Order.
    units: DecimalNumber,
    /// The price threshold specified for the Stop Order. The Stop
    /// Order will only be filled by a market price that is equal to
    /// or worse than this price.
    price: PriceValue,
    /// The worst market price that may be used to fill this Stop
    /// Order. If the market gaps and crosses through both the price
    /// and the priceBound, the Stop Order will be cancelled instead
    /// of being filled.
    price_bound: Option<PriceValue>,
    /// The time-in-force requested for the Stop Order.
    #[serde_inline_default("GTC")]
    time_in_force: TimeInForce,
    /// The date/time when the Stop Order will be cancelled if its
    /// timeInForce is “GTD”.
    gtd_time: Option<DateTime<Utc>>,
    /// Specification of how Positions in the Account are modified
    /// when the Order is filled.
    #[serde_inline_default("DEFAULT")]
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
    #[serde_inline_default("DEFAULT")]
    trigger_condition: OrderTriggerCondition,
    /// TakeProfitDetails specifies the details of a Take Profit
    /// Order to be created on behalf of a client. This may happen
    /// when an Order is filled that opens a Trade requiring a Take
    /// Profit, or when a Trade’s dependent Take Profit Order is
    /// modified directly through the Trade.
    take_profit_on_fill: Option<TakeProfitDetails>,
    /// StopLossDetails specifies the details of a Stop Loss Order
    /// to be created on behalf of a client. This may happen when
    /// an Order is filled that opens a Trade requiring a Stop Loss,
    /// or when a Trade’s dependent Stop Loss Order is modified
    /// directly through the Trade.
    stop_loss_on_fill: Option<StopLossDetails>,
    /// GuaranteedStopLossDetails specifies the details of a
    /// Guaranteed Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens
    /// a Trade requiring a Guaranteed Stop Loss, or when a Trade’s
    /// dependent Guaranteed Stop Loss Order is modified directly
    /// through the Trade.
    guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// TrailingStopLossDetails specifies the details of a Trailing
    /// Stop Loss Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade
    /// requiring a Trailing Stop Loss, or when a Trade’s dependent
    /// Trailing Stop Loss Order is modified directly through the
    /// Trade.
    trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the
    /// Order is filled (if such a Trade is created). Do not set,
    /// modify, or delete tradeClientExtensions if your account is
    /// associated with MT4.
    trade_client_extensions: Option<ClientExtensions>,
    /// ID of the Transaction that filled this Order (only provided
    /// when the Order’s state is FILLED)
    filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the
    /// Order’s state is FILLED)
    filled_time: Option<DateTime<Utc>>,
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
    cancelled_time: Option<DateTime<Utc>>,
    /// The ID of the Order that was replaced by this Order (only
    /// provided if this Order was created as part of a cancel/
    /// replace).
    replaces_order_id: Option<OrderID>,
    /// The ID of the Order that replaced this Order (only provided
    /// if this Order was cancelled as part of a cancel/replace).
    replaced_by_order_id: Option<OrderID>,
}
impl Default for StopOrder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            create_time: Default::default(),
            state: Default::default(),
            client_extensions: Default::default(),
            r#type: "STOP",
            instrument: Default::default(),
            units: Default::default(),
            price: Default::default(),
            price_bound: Default::default(),
            time_in_force: "GTC",
            gtd_time: Default::default(),
            position_fill: "DEFAULT",
            trigger_condition: "DEFAULT",
            take_profit_on_fill: Default::default(),
            stop_loss_on_fill: Default::default(),
            guaranteed_stop_loss_on_fill: Default::default(),
            trailing_stop_loss_on_fill: Default::default(),
            trade_client_extensions: Default::default(),
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
