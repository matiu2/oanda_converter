use serde :: { Serialize , Deserialize } ; _blank_ ! () ; # [derive (Serialize , Deserialize)] struct StopLossOrderRejectTransaction { # [doc = " The Transaction’s Identifier."] id : Option < TransactionID > , # [doc = " The date/time when the Transaction was created."] time : Option < DateTime > , # [doc = " The ID of the user that initiated the creation of the Transaction."] userID : Option < integer > , # [doc = " The ID of the Account the Transaction was created for."] accountID : Option < AccountID > , # [doc = " The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the Account simultaneously."] batchID : Option < TransactionID > , # [doc = " The Request ID of the request which generated the transaction."] requestID : Option < RequestID > , # [doc = " The Type of the Transaction. Always set to “STOP_LOSS_ORDER_REJECT” in a StopLossOrderRejectTransaction."] # [serde (default = "STOP_LOSS_ORDER_REJECT")] type : TransactionType , # [doc = " The ID of the Trade to close when the price threshold is breached."] tradeID : TradeID , # [doc = " The client ID of the Trade to be closed when the price threshold is breached."] clientTradeID : Option < ClientID > , # [doc = " The price threshold specified for the Stop Loss Order. The associated Trade will be closed by a market price that is equal to or worse than this threshold."] price : PriceValue , # [doc = " Specifies the distance (in price units) from the Account’s current price to use as the Stop Loss Order price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask is used."] distance : Option < DecimalNumber > , # [doc = " The time-in-force requested for the StopLoss Order. Restricted to “GTC”, “GFD” and “GTD” for StopLoss Orders."] # [serde (default = "GTC")] timeInForce : TimeInForce , # [doc = " The date/time when the StopLoss Order will be cancelled if its timeInForce is “GTD”."] gtdTime : Option < DateTime > , # [doc = " Specification of which price component should be used when determining if an Order should be triggered and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always filled using their default price component. This feature is only provided through the REST API. Clients who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s proprietary or partner trading platforms, their transaction history or their account statements. OANDA platforms always assume that an Order’s trigger condition is set to the default value when indicating the distance from an Order’s trigger price, and will always provide the default trigger condition when creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT” and “BID”, and for short trades “DEFAULT” and “ASK” are valid."] # [serde (default = "DEFAULT")] triggerCondition : OrderTriggerCondition , # [doc = " The reason that the Stop Loss Order was initiated"] reason : Option < StopLossOrderReason > , # [doc = " Client Extensions to add to the Order (only provided if the Order is being created with client extensions)."] clientExtensions : Option < ClientExtensions > , # [doc = " The ID of the OrderFill Transaction that caused this Order to be created (only provided if this Order was created automatically when another Order was filled)."] orderFillTransactionID : Option < TransactionID > , # [doc = " The ID of the Order that this Order was intended to replace (only provided if this Order was intended to replace an existing Order)."] intendedReplacesOrderID : Option < OrderID > , # [doc = " The reason that the Reject Transaction was created"] rejectReason : Option < TransactionRejectReason > , }