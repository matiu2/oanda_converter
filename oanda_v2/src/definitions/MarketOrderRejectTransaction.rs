use serde :: { Serialize , Deserialize } ; _blank_ ! () ; # [derive (Serialize , Deserialize)] struct MarketOrderRejectTransaction { # [doc = " The Transaction’s Identifier."] id : Option < TransactionID > , # [doc = " The date/time when the Transaction was created."] time : Option < DateTime > , # [doc = " The ID of the user that initiated the creation of the Transaction."] userID : Option < integer > , # [doc = " The ID of the Account the Transaction was created for."] accountID : Option < AccountID > , # [doc = " The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the Account simultaneously."] batchID : Option < TransactionID > , # [doc = " The Request ID of the request which generated the transaction."] requestID : Option < RequestID > , # [doc = " The Type of the Transaction. Always set to “MARKET_ORDER_REJECT” in a MarketOrderRejectTransaction."] # [serde (default = "MARKET_ORDER_REJECT")] type : TransactionType , # [doc = " The Market Order’s Instrument."] instrument : InstrumentName , # [doc = " The quantity requested to be filled by the Market Order. A positive number of units results in a long Order, and a negative number of units results in a short Order."] units : DecimalNumber , # [doc = " The time-in-force requested for the Market Order. Restricted to FOK or IOC for a MarketOrder."] # [serde (default = "FOK")] timeInForce : TimeInForce , # [doc = " The worst price that the client is willing to have the Market Order filled at."] priceBound : Option < PriceValue > , # [doc = " Specification of how Positions in the Account are modified when the Order is filled."] # [serde (default = "DEFAULT")] positionFill : OrderPositionFill , # [doc = " Details of the Trade requested to be closed, only provided when the Market Order is being used to explicitly close a Trade."] tradeClose : Option < MarketOrderTradeClose > , # [doc = " Details of the long Position requested to be closed out, only provided when a Market Order is being used to explicitly closeout a long Position."] longPositionCloseout : Option < MarketOrderPositionCloseout > , # [doc = " Details of the short Position requested to be closed out, only provided when a Market Order is being used to explicitly closeout a short Position."] shortPositionCloseout : Option < MarketOrderPositionCloseout > , # [doc = " Details of the Margin Closeout that this Market Order was created for"] marginCloseout : Option < MarketOrderMarginCloseout > , # [doc = " Details of the delayed Trade close that this Market Order was created for"] delayedTradeClose : Option < MarketOrderDelayedTradeClose > , # [doc = " The reason that the Market Order was created"] reason : Option < MarketOrderReason > , # [doc = " Client Extensions to add to the Order (only provided if the Order is being created with client extensions)."] clientExtensions : Option < ClientExtensions > , # [doc = " The specification of the Take Profit Order that should be created for a Trade opened when the Order is filled (if such a Trade is created)."] takeProfitOnFill : Option < TakeProfitDetails > , # [doc = " The specification of the Stop Loss Order that should be created for a Trade opened when the Order is filled (if such a Trade is created)."] stopLossOnFill : Option < StopLossDetails > , # [doc = " The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when the Order is filled (if such a Trade is created)."] trailingStopLossOnFill : Option < TrailingStopLossDetails > , # [doc = " The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened when the Order is filled (if such a Trade is created)."] guaranteedStopLossOnFill : Option < GuaranteedStopLossDetails > , # [doc = " Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created).  Do not set, modify, delete tradeClientExtensions if your account is associated with MT4."] tradeClientExtensions : Option < ClientExtensions > , # [doc = " The reason that the Reject Transaction was created"] rejectReason : Option < TransactionRejectReason > , }