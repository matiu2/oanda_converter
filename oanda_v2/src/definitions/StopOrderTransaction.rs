use serde :: { Serialize , Deserialize } ; _blank_ ! () ; # [derive (Serialize , Deserialize)] struct StopOrderTransaction { # [doc = " The Transaction’s Identifier."] id : Option < TransactionID > , # [doc = " The date/time when the Transaction was created."] time : Option < DateTime > , # [doc = " The ID of the user that initiated the creation of the Transaction."] userID : Option < integer > , # [doc = " The ID of the Account the Transaction was created for."] accountID : Option < AccountID > , # [doc = " The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the Account simultaneously."] batchID : Option < TransactionID > , # [doc = " The Request ID of the request which generated the transaction."] requestID : Option < RequestID > , # [doc = " The Type of the Transaction. Always set to “STOP_ORDER” in a StopOrderTransaction."] # [serde (default = "STOP_ORDER")] type : TransactionType , # [doc = " The Stop Order’s Instrument."] instrument : InstrumentName , # [doc = " The quantity requested to be filled by the Stop Order. A positive number of units results in a long Order, and a negative number of units results in a short Order."] units : DecimalNumber , # [doc = " The price threshold specified for the Stop Order. The Stop Order will only be filled by a market price that is equal to or worse than this price."] price : PriceValue , # [doc = " The worst market price that may be used to fill this Stop Order. If the market gaps and crosses through both the price and the priceBound, the Stop Order will be cancelled instead of being filled."] priceBound : Option < PriceValue > , # [doc = " The time-in-force requested for the Stop Order."] # [serde (default = "GTC")] timeInForce : TimeInForce , # [doc = " The date/time when the Stop Order will be cancelled if its timeInForce is “GTD”."] gtdTime : Option < DateTime > , # [doc = " Specification of how Positions in the Account are modified when the Order is filled."] # [serde (default = "DEFAULT")] positionFill : OrderPositionFill , # [doc = " Specification of which price component should be used when determining if an Order should be triggered and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always filled using their default price component. This feature is only provided through the REST API. Clients who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s proprietary or partner trading platforms, their transaction history or their account statements. OANDA platforms always assume that an Order’s trigger condition is set to the default value when indicating the distance from an Order’s trigger price, and will always provide the default trigger condition when creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT” and “BID”, and for short trades “DEFAULT” and “ASK” are valid."] # [serde (default = "DEFAULT")] triggerCondition : OrderTriggerCondition , # [doc = " The reason that the Stop Order was initiated"] reason : Option < StopOrderReason > , # [doc = " Client Extensions to add to the Order (only provided if the Order is being created with client extensions)."] clientExtensions : Option < ClientExtensions > , # [doc = " The specification of the Take Profit Order that should be created for a Trade opened when the Order is filled (if such a Trade is created)."] takeProfitOnFill : Option < TakeProfitDetails > , # [doc = " The specification of the Stop Loss Order that should be created for a Trade opened when the Order is filled (if such a Trade is created)."] stopLossOnFill : Option < StopLossDetails > , # [doc = " The specification of the Trailing Stop Loss Order that should be created for a Trade that is opened when the Order is filled (if such a Trade is created)."] trailingStopLossOnFill : Option < TrailingStopLossDetails > , # [doc = " The specification of the Guaranteed Stop Loss Order that should be created for a Trade that is opened when the Order is filled (if such a Trade is created)."] guaranteedStopLossOnFill : Option < GuaranteedStopLossDetails > , # [doc = " Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created).  Do not set, modify, delete tradeClientExtensions if your account is associated with MT4."] tradeClientExtensions : Option < ClientExtensions > , # [doc = " The ID of the Order that this Order replaces (only provided if this Order replaces an existing Order)."] replacesOrderID : Option < OrderID > , # [doc = " The ID of the Transaction that cancels the replaced Order (only provided if this Order replaces an existing Order)."] cancellingTransactionID : Option < TransactionID > , }