use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct MarketIfTouchedOrderRequest {
    /// The type of the Order to Create. Must be set to
    /// “MARKET_IF_TOUCHED” when creating a Market If Touched Order.
    #[serde(default = "MARKET_IF_TOUCHED")]
    r#type: OrderType,
    /// The MarketIfTouched Order’s Instrument.
    instrument: InstrumentName,
    /// The quantity requested to be filled by the MarketIfTouched
    /// Order. A positive number of units results in a long Order,
    /// and a negative number of units results in a short Order.
    units: DecimalNumber,
    /// The price threshold specified for the MarketIfTouched Order.
    /// The MarketIfTouched Order will only be filled by a market
    /// price that crosses this price from the direction of the
    /// market price at the time when the Order was created (the
    /// initialMarketPrice). Depending on the value of the Order’s
    /// price and initialMarketPrice, the MarketIfTouchedOrder will
    /// behave like a Limit or a Stop Order.
    price: PriceValue,
    /// The worst market price that may be used to fill this
    /// MarketIfTouched Order.
    price_bound: Option<PriceValue>,
    /// The time-in-force requested for the MarketIfTouched Order.
    /// Restricted to “GTC”, “GFD” and “GTD” for MarketIfTouched
    /// Orders.
    #[serde(default = "GTC")]
    time_in_force: TimeInForce,
    /// The date/time when the MarketIfTouched Order will be
    /// cancelled if its timeInForce is “GTD”.
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
    /// The client extensions to add to the Order. Do not set,
    /// modify, or delete clientExtensions if your account is
    /// associated with MT4.
    client_extensions: Option<ClientExtensions>,
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
}
