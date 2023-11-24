use serde::{Serialize, Deserialize};
/// The reason that an Order was cancelled.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum OrderCancelReason {
    /// The Order was cancelled because at the time of filling, an
    /// unexpected internal server error occurred.
    InternalServerError,
    /// The Order was cancelled because at the time of filling the
    /// account was locked.
    AccountLocked,
    /// The order was to be filled, however the account is
    /// configured to not allow new positions to be created.
    AccountNewPositionsLocked,
    /// Filling the Order wasn’t possible because it required the
    /// creation of a dependent Order and the Account is locked for
    /// Order creation.
    AccountOrderCreationLocked,
    /// Filling the Order was not possible because the Account is
    /// locked for filling Orders.
    AccountOrderFillLocked,
    /// The Order was cancelled explicitly at the request of the
    /// client.
    ClientRequest,
    /// The Order cancelled because it is being migrated to another
    /// account.
    Migration,
    /// Filling the Order wasn’t possible because the Order’s
    /// instrument was halted.
    MarketHalted,
    /// The Order is linked to an open Trade that was closed.
    LinkedTradeClosed,
    /// The time in force specified for this order has passed.
    TimeInForceExpired,
    /// Filling the Order wasn’t possible because the Account had
    /// insufficient margin.
    InsufficientMargin,
    /// Filling the Order would have resulted in a a FIFO violation.
    FifoViolation,
    /// Filling the Order would have violated the Order’s price
    /// bound.
    BoundsViolation,
    /// The Order was cancelled for replacement at the request of
    /// the client.
    ClientRequestReplaced,
    /// The Order was cancelled for replacement with an adjusted
    /// fillPrice to accommodate for the price movement caused by a
    /// dividendAdjustment.
    DividendAdjustmentReplaced,
    /// Filling the Order wasn’t possible because enough liquidity
    /// available.
    InsufficientLiquidity,
    /// Filling the Order would have resulted in the creation of a
    /// Take Profit Order with a GTD time in the past.
    TakeProfitOnFillGtdTimestampInPast,
    /// Filling the Order would result in the creation of a Take
    /// Profit Order that would have been filled immediately,
    /// closing the new Trade at a loss.
    TakeProfitOnFillLoss,
    /// Filling the Order would result in the creation of a Take
    /// Profit Loss Order that would close the new Trade at a loss
    /// when filled.
    LosingTakeProfit,
    /// Filling the Order would have resulted in the creation of a
    /// Stop Loss Order with a GTD time in the past.
    StopLossOnFillGtdTimestampInPast,
    /// Filling the Order would result in the creation of a Stop
    /// Loss Order that would have been filled immediately, closing
    /// the new Trade at a loss.
    StopLossOnFillLoss,
    /// Filling the Order would result in the creation of a Stop
    /// Loss Order whose price would be zero or negative due to the
    /// specified distance.
    StopLossOnFillPriceDistanceMaximumExceeded,
    /// Filling the Order would not result in the creation of Stop
    /// Loss Order, however the Account’s configuration requires
    /// that all Trades have a Stop Loss Order attached to them.
    StopLossOnFillRequired,
    /// Filling the Order would not result in the creation of
    /// a guaranteed Stop Loss Order, however the Account’s
    /// configuration requires that all Trades have a guaranteed
    /// Stop Loss Order attached to them.
    StopLossOnFillGuaranteedRequired,
    /// Filling the Order would result in the creation of
    /// a guaranteed Stop Loss Order, however the Account’s
    /// configuration does not allow guaranteed Stop Loss Orders.
    StopLossOnFillGuaranteedNotAllowed,
    /// Filling the Order would result in the creation of a
    /// guaranteed Stop Loss Order with a distance smaller than the
    /// configured minimum distance.
    StopLossOnFillGuaranteedMinimumDistanceNotMet,
    /// Filling the Order would result in the creation of a
    /// guaranteed Stop Loss Order with trigger price and number of
    /// units that that violates the account’s guaranteed Stop Loss
    /// Order level restriction.
    StopLossOnFillGuaranteedLevelRestrictionExceeded,
    /// Filling the Order would result in the creation of a
    /// guaranteed Stop Loss Order for a hedged Trade, however the
    /// Account’s configuration does not allow guaranteed Stop Loss
    /// Orders for hedged Trades/Positions.
    StopLossOnFillGuaranteedHedgingNotAllowed,
    /// Filling the Order would result in the creation of a Stop
    /// Loss Order whose TimeInForce value is invalid. A likely
    /// cause would be if the Account requires guaranteed stop loss
    /// orders and the TimeInForce value were not GTC.
    StopLossOnFillTimeInForceInvalid,
    /// Filling the Order would result in the creation of a Stop
    /// Loss Order whose TriggerCondition value is invalid. A likely
    /// cause would be if the stop loss order is guaranteed and the
    /// TimeInForce is not TRIGGER_DEFAULT or TRIGGER_BID for a long
    /// trade, or not TRIGGER_DEFAULT or TRIGGER_ASK for a short
    /// trade.
    StopLossOnFillTriggerConditionInvalid,
    /// Filling the Order would have resulted in the creation of a
    /// Guaranteed Stop Loss Order with a GTD time in the past.
    GuaranteedStopLossOnFillGtdTimestampInPast,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order that would have been filled
    /// immediately, closing the new Trade at a loss.
    GuaranteedStopLossOnFillLoss,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order whose price would be zero or
    /// negative due to the specified distance.
    GuaranteedStopLossOnFillPriceDistanceMaximumExceeded,
    /// Filling the Order would not result in the creation of
    /// a Guaranteed Stop Loss Order, however the Account’s
    /// configuration requires that all Trades have a Guaranteed
    /// Stop Loss Order attached to them.
    GuaranteedStopLossOnFillRequired,
    /// Filling the Order would result in the creation of
    /// a Guaranteed Stop Loss Order, however the Account’s
    /// configuration does not allow Guaranteed Stop Loss Orders.
    GuaranteedStopLossOnFillNotAllowed,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order with a distance smaller than the
    /// configured minimum distance.
    GuaranteedStopLossOnFillMinimumDistanceNotMet,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order with trigger number of units
    /// that violates the account’s Guaranteed Stop Loss Order level
    /// restriction volume.
    GuaranteedStopLossOnFillLevelRestrictionVolumeExceeded,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order with trigger price that violates
    /// the account’s Guaranteed Stop Loss Order level restriction
    /// price range.
    GuaranteedStopLossOnFillLevelRestrictionPriceRangeExceeded,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order for a hedged Trade, however the
    /// Account’s configuration does not allow Guaranteed Stop Loss
    /// Orders for hedged Trades/Positions.
    GuaranteedStopLossOnFillHedgingNotAllowed,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order whose TimeInForce value is
    /// invalid. A likely cause would be if the Account requires
    /// guaranteed stop loss orders and the TimeInForce value were
    /// not GTC.
    GuaranteedStopLossOnFillTimeInForceInvalid,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order whose TriggerCondition value
    /// is invalid. A likely cause would be the TimeInForce is not
    /// TRIGGER_DEFAULT or TRIGGER_BID for a long trade, or not
    /// TRIGGER_DEFAULT or TRIGGER_ASK for a short trade.
    GuaranteedStopLossOnFillTriggerConditionInvalid,
    /// Filling the Order would result in the creation of a Take
    /// Profit Order whose price would be zero or negative due to
    /// the specified distance.
    TakeProfitOnFillPriceDistanceMaximumExceeded,
    /// Filling the Order would have resulted in the creation of a
    /// Trailing Stop Loss Order with a GTD time in the past.
    TrailingStopLossOnFillGtdTimestampInPast,
    /// Filling the Order would result in the creation of a new Open
    /// Trade with a client Trade ID already in use.
    ClientTradeIdAlreadyExists,
    /// Closing out a position wasn’t fully possible.
    PositionCloseoutFailed,
    /// Filling the Order would cause the maximum open trades
    /// allowed for the Account to be exceeded.
    OpenTradesAllowedExceeded,
    /// Filling the Order would have resulted in exceeding the
    /// number of pending Orders allowed for the Account.
    PendingOrdersAllowedExceeded,
    /// Filling the Order would have resulted in the creation of
    /// a Take Profit Order with a client Order ID that is already
    /// in use.
    TakeProfitOnFillClientOrderIdAlreadyExists,
    /// Filling the Order would have resulted in the creation of
    /// a Stop Loss Order with a client Order ID that is already
    /// in use.
    StopLossOnFillClientOrderIdAlreadyExists,
    /// Filling the Order would have resulted in the creation of a
    /// Guaranteed Stop Loss Order with a client Order ID that is
    /// already in use.
    GuaranteedStopLossOnFillClientOrderIdAlreadyExists,
    /// Filling the Order would have resulted in the creation of
    /// a Trailing Stop Loss Order with a client Order ID that is
    /// already in use.
    TrailingStopLossOnFillClientOrderIdAlreadyExists,
    /// Filling the Order would have resulted in the Account’s
    /// maximum position size limit being exceeded for the Order’s
    /// instrument.
    PositionSizeExceeded,
    /// Filling the Order would result in the creation of a Trade,
    /// however there already exists an opposing (hedged) Trade that
    /// has a guaranteed Stop Loss Order attached to it. Guaranteed
    /// Stop Loss Orders cannot be combined with hedged positions.
    HedgingGsloViolation,
    /// Filling the order would cause the maximum position value
    /// allowed for the account to be exceeded. The Order has been
    /// cancelled as a result.
    AccountPositionValueLimitExceeded,
    /// Filling the order would require the creation of a short
    /// trade, however the instrument is configured such that orders
    /// being filled using bid prices can only reduce existing
    /// positions. New short positions cannot be created, but
    /// existing long positions may be reduced or closed.
    InstrumentBidReduceOnly,
    /// Filling the order would require the creation of a long
    /// trade, however the instrument is configured such that orders
    /// being filled using ask prices can only reduce existing
    /// positions. New long positions cannot be created, but
    /// existing short positions may be reduced or closed.
    InstrumentAskReduceOnly,
    /// Filling the order would require using the bid, however the
    /// instrument is configured such that the bids are halted, and
    /// so no short orders may be filled.
    InstrumentBidHalted,
    /// Filling the order would require using the ask, however the
    /// instrument is configured such that the asks are halted, and
    /// so no long orders may be filled.
    InstrumentAskHalted,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order (GSLO). Since the trade is long
    /// the GSLO would be short, however the bid side is currently
    /// halted. GSLOs cannot be created in this situation.
    StopLossOnFillGuaranteedBidHalted,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order (GSLO). Since the trade is short
    /// the GSLO would be long, however the ask side is currently
    /// halted. GSLOs cannot be created in this situation.
    StopLossOnFillGuaranteedAskHalted,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order (GSLO). Since the trade is long
    /// the GSLO would be short, however the bid side is currently
    /// halted. GSLOs cannot be created in this situation.
    GuaranteedStopLossOnFillBidHalted,
    /// Filling the Order would result in the creation of a
    /// Guaranteed Stop Loss Order (GSLO). Since the trade is short
    /// the GSLO would be long, however the ask side is currently
    /// halted. GSLOs cannot be created in this situation.
    GuaranteedStopLossOnFillAskHalted,
    /// Filling the Order would have resulted in a new Trade that
    /// violates the FIFO violation safeguard constraints.
    FifoViolationSafeguardViolation,
    /// Filling the Order would have reduced an existing Trade such
    /// that the reduced Trade violates the FIFO violation safeguard
    /// constraints.
    FifoViolationSafeguardPartialCloseViolation,
    /// The Orders on fill would be in violation of the risk
    /// management Order mutual exclusivity configuration specifying
    /// that only one risk management Order can be attached to
    /// a Trade.
    OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation,
}
