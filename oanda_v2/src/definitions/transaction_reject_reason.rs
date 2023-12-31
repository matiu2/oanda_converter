use serde::{Serialize, Deserialize};
/// The reason that a Transaction was rejected.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionRejectReason {
    /// An unexpected internal server error has occurred
    InternalServerError,
    /// The system was unable to determine the current price for the
    /// Order’s instrument
    InstrumentPriceUnknown,
    /// The Account is not active
    AccountNotActive,
    /// The Account is locked
    AccountLocked,
    /// The Account is locked for Order creation
    AccountOrderCreationLocked,
    /// The Account is locked for configuration
    AccountConfigurationLocked,
    /// The Account is locked for deposits
    AccountDepositLocked,
    /// The Account is locked for withdrawals
    AccountWithdrawalLocked,
    /// The Account is locked for Order cancellation
    AccountOrderCancelLocked,
    /// The instrument specified is not tradeable by the Account
    InstrumentNotTradeable,
    /// Creating the Order would result in the maximum number of
    /// allowed pending Orders being exceeded
    PendingOrdersAllowedExceeded,
    /// Neither the Order ID nor client Order ID are specified
    OrderIdUnspecified,
    /// The Order specified does not exist
    OrderDoesntExist,
    /// The Order ID and client Order ID specified do not identify
    /// the same Order
    OrderIdentifierInconsistency,
    /// Neither the Trade ID nor client Trade ID are specified
    TradeIdUnspecified,
    /// The Trade specified does not exist
    TradeDoesntExist,
    /// The Trade ID and client Trade ID specified do not identify
    /// the same Trade
    TradeIdentifierInconsistency,
    /// The Account had insufficient margin to perform the action
    /// specified. One possible reason for this is due to the
    /// creation or modification of a guaranteed StopLoss Order.
    InsufficientMargin,
    /// Order instrument has not been specified
    InstrumentMissing,
    /// The instrument specified is unknown
    InstrumentUnknown,
    /// Order units have not been not specified
    UnitsMissing,
    /// Order units specified are invalid
    UnitsInvalid,
    /// The units specified contain more precision than is allowed
    /// for the Order’s instrument
    UnitsPrecisionExceeded,
    /// The units specified exceeds the maximum number of units
    /// allowed
    UnitsLimitExceeded,
    /// The units specified is less than the minimum number of units
    /// required
    UnitsMinimumNotMet,
    /// The price has not been specified
    PriceMissing,
    /// The price specified is invalid
    PriceInvalid,
    /// The price specified contains more precision than is allowed
    /// for the instrument
    PricePrecisionExceeded,
    /// The price distance has not been specified
    PriceDistanceMissing,
    /// The price distance specified is invalid
    PriceDistanceInvalid,
    /// The price distance specified contains more precision than is
    /// allowed for the instrument
    PriceDistancePrecisionExceeded,
    /// The price distance exceeds that maximum allowed amount
    PriceDistanceMaximumExceeded,
    /// The price distance does not meet the minimum allowed amount
    PriceDistanceMinimumNotMet,
    /// The TimeInForce field has not been specified
    TimeInForceMissing,
    /// The TimeInForce specified is invalid
    TimeInForceInvalid,
    /// The TimeInForce is GTD but no GTD timestamp is provided
    TimeInForceGtdTimestampMissing,
    /// The TimeInForce is GTD but the GTD timestamp is in the past
    TimeInForceGtdTimestampInPast,
    /// The price bound specified is invalid
    PriceBoundInvalid,
    /// The price bound specified contains more precision than is
    /// allowed for the Order’s instrument
    PriceBoundPrecisionExceeded,
    /// Multiple Orders on fill share the same client Order ID
    OrdersOnFillDuplicateClientOrderIds,
    /// The Order does not support Trade on fill client extensions
    /// because it cannot create a new Trade
    TradeOnFillClientExtensionsNotSupported,
    /// The client Order ID specified is invalid
    ClientOrderIdInvalid,
    /// The client Order ID specified is already assigned to another
    /// pending Order
    ClientOrderIdAlreadyExists,
    /// The client Order tag specified is invalid
    ClientOrderTagInvalid,
    /// The client Order comment specified is invalid
    ClientOrderCommentInvalid,
    /// The client Trade ID specified is invalid
    ClientTradeIdInvalid,
    /// The client Trade ID specified is already assigned to another
    /// open Trade
    ClientTradeIdAlreadyExists,
    /// The client Trade tag specified is invalid
    ClientTradeTagInvalid,
    /// The client Trade comment is invalid
    ClientTradeCommentInvalid,
    /// The OrderFillPositionAction field has not been specified
    OrderFillPositionActionMissing,
    /// The OrderFillPositionAction specified is invalid
    OrderFillPositionActionInvalid,
    /// The TriggerCondition field has not been specified
    TriggerConditionMissing,
    /// The TriggerCondition specified is invalid
    TriggerConditionInvalid,
    /// The OrderFillPositionAction field has not been specified
    OrderPartialFillOptionMissing,
    /// The OrderFillPositionAction specified is invalid.
    OrderPartialFillOptionInvalid,
    /// When attempting to reissue an order (currently only a
    /// MarketIfTouched) that was immediately partially filled, it
    /// is not possible to create a correct pending Order.
    InvalidReissueImmediatePartialFill,
    /// The Orders on fill would be in violation of the risk
    /// management Order mutual exclusivity configuration specifying
    /// that only one risk management Order can be attached to
    /// a Trade.
    OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation,
    /// The Orders on fill would be in violation of the risk
    /// management Order mutual exclusivity configuration specifying
    /// that if a GSLO is already attached to a Trade, no other risk
    /// management Order can be attached to a Trade.
    OrdersOnFillRmoMutualExclusivityGsloExcludesOthersViolation,
    /// A Take Profit Order for the specified Trade already exists
    TakeProfitOrderAlreadyExists,
    /// The Take Profit Order would cause the associated Trade to be
    /// in violation of the FIFO violation safeguard constraints.
    TakeProfitOrderWouldViolateFifoViolationSafeguard,
    /// The Take Profit on fill specified does not provide a price
    TakeProfitOnFillPriceMissing,
    /// The Take Profit on fill specified contains an invalid price
    TakeProfitOnFillPriceInvalid,
    /// The Take Profit on fill specified contains a price with more
    /// precision than is allowed by the Order’s instrument
    TakeProfitOnFillPricePrecisionExceeded,
    /// The Take Profit on fill specified does not provide a
    /// TimeInForce
    TakeProfitOnFillTimeInForceMissing,
    /// The Take Profit on fill specifies an invalid TimeInForce
    TakeProfitOnFillTimeInForceInvalid,
    /// The Take Profit on fill specifies a GTD TimeInForce but does
    /// not provide a GTD timestamp
    TakeProfitOnFillGtdTimestampMissing,
    /// The Take Profit on fill specifies a GTD timestamp that is in
    /// the past
    TakeProfitOnFillGtdTimestampInPast,
    /// The Take Profit on fill client Order ID specified is invalid
    TakeProfitOnFillClientOrderIdInvalid,
    /// The Take Profit on fill client Order tag specified is
    /// invalid
    TakeProfitOnFillClientOrderTagInvalid,
    /// The Take Profit on fill client Order comment specified is
    /// invalid
    TakeProfitOnFillClientOrderCommentInvalid,
    /// The Take Profit on fill specified does not provide a
    /// TriggerCondition
    TakeProfitOnFillTriggerConditionMissing,
    /// The Take Profit on fill specifies an invalid
    /// TriggerCondition
    TakeProfitOnFillTriggerConditionInvalid,
    /// A Stop Loss Order for the specified Trade already exists
    StopLossOrderAlreadyExists,
    /// An attempt was made to to create a non-guaranteed stop loss
    /// order in an account that requires all stop loss orders to
    /// be guaranteed.
    StopLossOrderGuaranteedRequired,
    /// An attempt to create a guaranteed stop loss order with a
    /// price that is within the current tradeable spread.
    StopLossOrderGuaranteedPriceWithinSpread,
    /// An attempt was made to create a guaranteed Stop Loss
    /// Order, however the Account’s configuration does not allow
    /// guaranteed Stop Loss Orders.
    StopLossOrderGuaranteedNotAllowed,
    /// An attempt was made to create a guaranteed Stop Loss Order
    /// when the market was halted.
    StopLossOrderGuaranteedHaltedCreateViolation,
    /// An attempt was made to re-create a guaranteed Stop Loss
    /// Order with a tighter fill price when the market was halted.
    StopLossOrderGuaranteedHaltedTightenViolation,
    /// An attempt was made to create a guaranteed Stop Loss Order
    /// on a hedged Trade (ie there is an existing open Trade in
    /// the opposing direction), however the Account’s configuration
    /// does not allow guaranteed Stop Loss Orders for hedged
    /// Trades/Positions.
    StopLossOrderGuaranteedHedgingNotAllowed,
    /// An attempt was made to create a guaranteed Stop Loss Order,
    /// however the distance between the current price and the
    /// trigger price does not meet the Account’s configured minimum
    /// Guaranteed Stop Loss distance.
    StopLossOrderGuaranteedMinimumDistanceNotMet,
    /// An attempt was made to cancel a Stop Loss Order, however
    /// the Account’s configuration requires every Trade have an
    /// associated Stop Loss Order.
    StopLossOrderNotCancelable,
    /// An attempt was made to cancel and replace a Stop Loss
    /// Order, however the Account’s configuration prevents the
    /// modification of Stop Loss Orders.
    StopLossOrderNotReplaceable,
    /// An attempt was made to create a guaranteed Stop Loss Order,
    /// however doing so would exceed the Account’s configured
    /// guaranteed StopLoss Order level restriction volume.
    StopLossOrderGuaranteedLevelRestrictionExceeded,
    /// The Stop Loss Order request contains both the price and
    /// distance fields.
    StopLossOrderPriceAndDistanceBothSpecified,
    /// The Stop Loss Order request contains neither the price nor
    /// distance fields.
    StopLossOrderPriceAndDistanceBothMissing,
    /// The Stop Loss Order would cause the associated Trade to be
    /// in violation of the FIFO violation safeguard constraints
    StopLossOrderWouldViolateFifoViolationSafeguard,
    /// The Stop Loss Order would be in violation of the risk
    /// management Order mutual exclusivity configuration specifying
    /// that only one risk management order can be attached to
    /// a Trade.
    StopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation,
    /// The Stop Loss Order would be in violation of the risk
    /// management Order mutual exclusivity configuration specifying
    /// that if a GSLO is already attached to a Trade, no other risk
    /// management Order can be attached to the same Trade.
    StopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation,
    /// An attempt to create a pending Order was made with no Stop
    /// Loss Order on fill specified and the Account’s configuration
    /// requires that every Trade have an associated Stop Loss
    /// Order.
    StopLossOnFillRequiredForPendingOrder,
    /// An attempt to create a pending Order was made with a Stop
    /// Loss Order on fill that was explicitly configured to be
    /// guaranteed, however the Account’s configuration does not
    /// allow guaranteed Stop Loss Orders.
    StopLossOnFillGuaranteedNotAllowed,
    /// An attempt to create a pending Order was made with a Stop
    /// Loss Order on fill that was explicitly configured to be not
    /// guaranteed, however the Account’s configuration requires
    /// guaranteed Stop Loss Orders.
    StopLossOnFillGuaranteedRequired,
    /// The Stop Loss on fill specified does not provide a price
    StopLossOnFillPriceMissing,
    /// The Stop Loss on fill specifies an invalid price
    StopLossOnFillPriceInvalid,
    /// The Stop Loss on fill specifies a price with more precision
    /// than is allowed by the Order’s instrument
    StopLossOnFillPricePrecisionExceeded,
    /// An attempt to create a pending Order was made with the
    /// distance between the guaranteed Stop Loss Order on fill’s
    /// price and the pending Order’s price is less than the
    /// Account’s configured minimum guaranteed stop loss distance.
    StopLossOnFillGuaranteedMinimumDistanceNotMet,
    /// An attempt to create a pending Order was made with a
    /// guaranteed Stop Loss Order on fill configured, and the
    /// Order’s units exceed the Account’s configured guaranteed
    /// StopLoss Order level restriction volume.
    StopLossOnFillGuaranteedLevelRestrictionExceeded,
    /// The Stop Loss on fill distance is invalid
    StopLossOnFillDistanceInvalid,
    /// The Stop Loss on fill price distance exceeds the maximum
    /// allowed amount
    StopLossOnFillPriceDistanceMaximumExceeded,
    /// The Stop Loss on fill distance contains more precision than
    /// is allowed by the instrument
    StopLossOnFillDistancePrecisionExceeded,
    /// The Stop Loss on fill contains both the price and distance
    /// fields.
    StopLossOnFillPriceAndDistanceBothSpecified,
    /// The Stop Loss on fill contains neither the price nor
    /// distance fields.
    StopLossOnFillPriceAndDistanceBothMissing,
    /// The Stop Loss on fill specified does not provide a
    /// TimeInForce
    StopLossOnFillTimeInForceMissing,
    /// The Stop Loss on fill specifies an invalid TimeInForce
    StopLossOnFillTimeInForceInvalid,
    /// The Stop Loss on fill specifies a GTD TimeInForce but does
    /// not provide a GTD timestamp
    StopLossOnFillGtdTimestampMissing,
    /// The Stop Loss on fill specifies a GTD timestamp that is in
    /// the past
    StopLossOnFillGtdTimestampInPast,
    /// The Stop Loss on fill client Order ID specified is invalid
    StopLossOnFillClientOrderIdInvalid,
    /// The Stop Loss on fill client Order tag specified is invalid
    StopLossOnFillClientOrderTagInvalid,
    /// The Stop Loss on fill client Order comment specified is
    /// invalid
    StopLossOnFillClientOrderCommentInvalid,
    /// The Stop Loss on fill specified does not provide a
    /// TriggerCondition
    StopLossOnFillTriggerConditionMissing,
    /// The Stop Loss on fill specifies an invalid TriggerCondition
    StopLossOnFillTriggerConditionInvalid,
    /// A Guaranteed Stop Loss Order for the specified Trade already
    /// exists
    GuaranteedStopLossOrderAlreadyExists,
    /// An attempt was made to to create a non-guaranteed stop loss
    /// order in an account that requires all stop loss orders to
    /// be guaranteed.
    GuaranteedStopLossOrderRequired,
    /// An attempt to create a guaranteed stop loss order with a
    /// price that is within the current tradeable spread.
    GuaranteedStopLossOrderPriceWithinSpread,
    /// An attempt was made to create a Guaranteed Stop Loss
    /// Order, however the Account’s configuration does not allow
    /// Guaranteed Stop Loss Orders.
    GuaranteedStopLossOrderNotAllowed,
    /// An attempt was made to create a Guaranteed Stop Loss Order
    /// when the market was halted.
    GuaranteedStopLossOrderHaltedCreateViolation,
    /// An attempt was made to create a Guaranteed Stop Loss Order
    /// when the market was open.
    GuaranteedStopLossOrderCreateViolation,
    /// An attempt was made to re-create a Guaranteed Stop Loss
    /// Order with a tighter fill price when the market was halted.
    GuaranteedStopLossOrderHaltedTightenViolation,
    /// An attempt was made to re-create a Guaranteed Stop Loss
    /// Order with a tighter fill price when the market was open.
    GuaranteedStopLossOrderTightenViolation,
    /// An attempt was made to create a Guaranteed Stop Loss Order
    /// on a hedged Trade (ie there is an existing open Trade in
    /// the opposing direction), however the Account’s configuration
    /// does not allow Guaranteed Stop Loss Orders for hedged
    /// Trades/Positions.
    GuaranteedStopLossOrderHedgingNotAllowed,
    /// An attempt was made to create a Guaranteed Stop Loss Order,
    /// however the distance between the current price and the
    /// trigger price does not meet the Account’s configured minimum
    /// Guaranteed Stop Loss distance.
    GuaranteedStopLossOrderMinimumDistanceNotMet,
    /// An attempt was made to cancel a Guaranteed Stop Loss Order
    /// when the market is open, however the Account’s configuration
    /// requires every Trade have an associated Guaranteed Stop
    /// Loss Order.
    GuaranteedStopLossOrderNotCancelable,
    /// An attempt was made to cancel a Guaranteed Stop Loss
    /// Order when the market is halted, however the Account’s
    /// configuration requires every Trade have an associated
    /// Guaranteed Stop Loss Order.
    GuaranteedStopLossOrderHaltedNotCancelable,
    /// An attempt was made to cancel and replace a Guaranteed Stop
    /// Loss Order when the market is open, however the Account’s
    /// configuration prevents the modification of Guaranteed Stop
    /// Loss Orders.
    GuaranteedStopLossOrderNotReplaceable,
    /// An attempt was made to cancel and replace a Guaranteed Stop
    /// Loss Order when the market is halted, however the Account’s
    /// configuration prevents the modification of Guaranteed Stop
    /// Loss Orders.
    GuaranteedStopLossOrderHaltedNotReplaceable,
    /// An attempt was made to create a Guaranteed Stop Loss Order,
    /// however doing so would exceed the Account’s configured
    /// guaranteed StopLoss Order level restriction volume.
    GuaranteedStopLossOrderLevelRestrictionVolumeExceeded,
    /// An attempt was made to create a Guaranteed Stop Loss Order,
    /// however doing so would exceed the Account’s configured
    /// guaranteed StopLoss Order level restriction price range.
    GuaranteedStopLossOrderLevelRestrictionPriceRangeExceeded,
    /// The Guaranteed Stop Loss Order request contains both the
    /// price and distance fields.
    GuaranteedStopLossOrderPriceAndDistanceBothSpecified,
    /// The Guaranteed Stop Loss Order request contains neither the
    /// price nor distance fields.
    GuaranteedStopLossOrderPriceAndDistanceBothMissing,
    /// The Guaranteed Stop Loss Order would cause the associated
    /// Trade to be in violation of the FIFO violation safeguard
    /// constraints
    GuaranteedStopLossOrderWouldViolateFifoViolationSafeguard,
    /// The Guaranteed Stop Loss Order would be in violation of
    /// the risk management Order mutual exclusivity configuration
    /// specifying that only one risk management order can be
    /// attached to a Trade.
    GuaranteedStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation,
    /// The Guaranteed Stop Loss Order would be in violation of
    /// the risk management Order mutual exclusivity configuration
    /// specifying that if a GSLO is already attached to a Trade,
    /// no other risk management Order can be attached to the same
    /// Trade.
    GuaranteedStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation,
    /// An attempt to create a pending Order was made with no
    /// Guaranteed Stop Loss Order on fill specified and the
    /// Account’s configuration requires that every Trade have an
    /// associated Guaranteed Stop Loss Order.
    GuaranteedStopLossOnFillRequiredForPendingOrder,
    /// An attempt to create a pending Order was made with a
    /// Guaranteed Stop Loss Order on fill that was explicitly
    /// configured to be guaranteed, however the Account’s
    /// configuration does not allow guaranteed Stop Loss Orders.
    GuaranteedStopLossOnFillNotAllowed,
    /// An attempt to create a pending Order was made with a
    /// Guaranteed Stop Loss Order on fill that was explicitly
    /// configured to be not guaranteed, however the Account’s
    /// configuration requires Guaranteed Stop Loss Orders.
    GuaranteedStopLossOnFillRequired,
    /// The Guaranteed Stop Loss on fill specified does not provide
    /// a price
    GuaranteedStopLossOnFillPriceMissing,
    /// The Guaranteed Stop Loss on fill specifies an invalid price
    GuaranteedStopLossOnFillPriceInvalid,
    /// The Guaranteed Stop Loss on fill specifies a price with more
    /// precision than is allowed by the Order’s instrument
    GuaranteedStopLossOnFillPricePrecisionExceeded,
    /// An attempt to create a pending Order was made with the
    /// distance between the Guaranteed Stop Loss Order on fill’s
    /// price and the pending Order’s price is less than the
    /// Account’s configured minimum guaranteed stop loss distance.
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
    /// The Guaranteed Stop Loss on fill distance is invalid
    GuaranteedStopLossOnFillDistanceInvalid,
    /// The Guaranteed Stop Loss on fill price distance exceeds the
    /// maximum allowed amount.
    GuaranteedStopLossOnFillPriceDistanceMaximumExceeded,
    /// The Guaranteed Stop Loss on fill distance contains more
    /// precision than is allowed by the instrument
    GuaranteedStopLossOnFillDistancePrecisionExceeded,
    /// The Guaranteed Stop Loss on fill contains both the price and
    /// distance fields.
    GuaranteedStopLossOnFillPriceAndDistanceBothSpecified,
    /// The Guaranteed Stop Loss on fill contains neither the price
    /// nor distance fields.
    GuaranteedStopLossOnFillPriceAndDistanceBothMissing,
    /// The Guaranteed Stop Loss on fill specified does not provide
    /// a TimeInForce
    GuaranteedStopLossOnFillTimeInForceMissing,
    /// The Guaranteed Stop Loss on fill specifies an invalid
    /// TimeInForce
    GuaranteedStopLossOnFillTimeInForceInvalid,
    /// The Guaranteed Stop Loss on fill specifies a GTD TimeInForce
    /// but does not provide a GTD timestamp
    GuaranteedStopLossOnFillGtdTimestampMissing,
    /// The Guaranteed Stop Loss on fill specifies a GTD timestamp
    /// that is in the past.
    GuaranteedStopLossOnFillGtdTimestampInPast,
    /// The Guaranteed Stop Loss on fill client Order ID specified
    /// is invalid
    GuaranteedStopLossOnFillClientOrderIdInvalid,
    /// The Guaranteed Stop Loss on fill client Order tag specified
    /// is invalid
    GuaranteedStopLossOnFillClientOrderTagInvalid,
    /// The Guaranteed Stop Loss on fill client Order comment
    /// specified is invalid.
    GuaranteedStopLossOnFillClientOrderCommentInvalid,
    /// The Guaranteed Stop Loss on fill specified does not provide
    /// a TriggerCondition.
    GuaranteedStopLossOnFillTriggerConditionMissing,
    /// The Guaranteed Stop Loss on fill specifies an invalid
    /// TriggerCondition.
    GuaranteedStopLossOnFillTriggerConditionInvalid,
    /// A Trailing Stop Loss Order for the specified Trade already
    /// exists
    TrailingStopLossOrderAlreadyExists,
    /// The Trailing Stop Loss Order would cause the associated
    /// Trade to be in violation of the FIFO violation safeguard
    /// constraints
    TrailingStopLossOrderWouldViolateFifoViolationSafeguard,
    /// The Trailing Stop Loss Order would be in violation of the
    /// risk management Order mutual exclusivity configuration
    /// specifying that only one risk management order can be
    /// attached to a Trade.
    TrailingStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation,
    /// The Trailing Stop Loss Order would be in violation of the
    /// risk management Order mutual exclusivity configuration
    /// specifying that if a GSLO is already attached to a Trade,
    /// no other risk management Order can be attached to the same
    /// Trade.
    TrailingStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation,
    /// The Trailing Stop Loss on fill specified does not provide
    /// a distance
    TrailingStopLossOnFillPriceDistanceMissing,
    /// The Trailing Stop Loss on fill distance is invalid
    TrailingStopLossOnFillPriceDistanceInvalid,
    /// The Trailing Stop Loss on fill distance contains more
    /// precision than is allowed by the instrument
    TrailingStopLossOnFillPriceDistancePrecisionExceeded,
    /// The Trailing Stop Loss on fill price distance exceeds the
    /// maximum allowed amount
    TrailingStopLossOnFillPriceDistanceMaximumExceeded,
    /// The Trailing Stop Loss on fill price distance does not meet
    /// the minimum allowed amount
    TrailingStopLossOnFillPriceDistanceMinimumNotMet,
    /// The Trailing Stop Loss on fill specified does not provide
    /// a TimeInForce
    TrailingStopLossOnFillTimeInForceMissing,
    /// The Trailing Stop Loss on fill specifies an invalid
    /// TimeInForce
    TrailingStopLossOnFillTimeInForceInvalid,
    /// The Trailing Stop Loss on fill TimeInForce is specified as
    /// GTD but no GTD timestamp is provided
    TrailingStopLossOnFillGtdTimestampMissing,
    /// The Trailing Stop Loss on fill GTD timestamp is in the past
    TrailingStopLossOnFillGtdTimestampInPast,
    /// The Trailing Stop Loss on fill client Order ID specified
    /// is invalid
    TrailingStopLossOnFillClientOrderIdInvalid,
    /// The Trailing Stop Loss on fill client Order tag specified
    /// is invalid
    TrailingStopLossOnFillClientOrderTagInvalid,
    /// The Trailing Stop Loss on fill client Order comment
    /// specified is invalid
    TrailingStopLossOnFillClientOrderCommentInvalid,
    /// A client attempted to create either a Trailing Stop
    /// Loss order or an order with a Trailing Stop Loss On Fill
    /// specified, which may not yet be supported.
    TrailingStopLossOrdersNotSupported,
    /// The Trailing Stop Loss on fill specified does not provide a
    /// TriggerCondition
    TrailingStopLossOnFillTriggerConditionMissing,
    /// The Tailing Stop Loss on fill specifies an invalid
    /// TriggerCondition
    TrailingStopLossOnFillTriggerConditionInvalid,
    /// The request to close a Trade does not specify a full or
    /// partial close
    CloseTradeTypeMissing,
    /// The request to close a Trade partially did not specify the
    /// number of units to close
    CloseTradePartialUnitsMissing,
    /// The request to partially close a Trade specifies a number of
    /// units that exceeds the current size of the given Trade
    CloseTradeUnitsExceedTradeSize,
    /// The Position requested to be closed out does not exist
    CloseoutPositionDoesntExist,
    /// The request to closeout a Position was specified
    /// incompletely
    CloseoutPositionIncompleteSpecification,
    /// A partial Position closeout request specifies a number of
    /// units that exceeds the current Position
    CloseoutPositionUnitsExceedPositionSize,
    /// The request to closeout a Position could not be fully
    /// satisfied
    CloseoutPositionReject,
    /// The request to partially closeout a Position did not specify
    /// the number of units to close.
    CloseoutPositionPartialUnitsMissing,
    /// The markup group ID provided is invalid
    MarkupGroupIdInvalid,
    /// The PositionAggregationMode provided is not supported/valid.
    PositionAggregationModeInvalid,
    /// No configuration parameters provided
    AdminConfigureDataMissing,
    /// The margin rate provided is invalid
    MarginRateInvalid,
    /// The margin rate provided would cause an immediate margin
    /// closeout
    MarginRateWouldTriggerCloseout,
    /// The account alias string provided is invalid
    AliasInvalid,
    /// No configuration parameters provided
    ClientConfigureDataMissing,
    /// The margin rate provided would cause the Account to enter a
    /// margin call state.
    MarginRateWouldTriggerMarginCall,
    /// Funding is not possible because the requested transfer
    /// amount is invalid
    AmountInvalid,
    /// The Account does not have sufficient balance to complete the
    /// funding request
    InsufficientFunds,
    /// Funding amount has not been specified
    AmountMissing,
    /// Funding reason has not been specified
    FundingReasonMissing,
    /// The list of Order Identifiers provided for a One Cancels
    /// All Order contains an Order Identifier that refers to a Stop
    /// Loss Order. OCA groups cannot contain Stop Loss Orders.
    OcaOrderIdsStopLossNotAllowed,
    /// Neither Order nor Trade on Fill client extensions were
    /// provided for modification
    ClientExtensionsDataMissing,
    /// The Order to be replaced has a different type than the
    /// replacing Order.
    ReplacingOrderInvalid,
    /// The replacing Order refers to a different Trade than the
    /// Order that is being replaced.
    ReplacingTradeIdInvalid,
    /// Canceling the order would cause an immediate margin
    /// closeout.
    OrderCancelWouldTriggerCloseout,
}
