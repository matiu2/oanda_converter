use serde :: { Serialize , Deserialize } ; _blank_ ! () ; # [derive (Serialize , Deserialize)] struct TakeProfitOrder { # [doc = " The Order’s identifier, unique within the Order’s Account."] id : Option < OrderID > , # [doc = " The time when the Order was created."] createTime : Option < DateTime > , # [doc = " The current state of the Order."] state : Option < OrderState > , # [doc = " The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is associated with MT4."] clientExtensions : Option < ClientExtensions > , # [doc = " The type of the Order. Always set to “TAKE_PROFIT” for Take Profit Orders."] # [serde (default = "TAKE_PROFIT")] type : OrderType , # [doc = " The ID of the Trade to close when the price threshold is breached."] tradeID : TradeID , # [doc = " The client ID of the Trade to be closed when the price threshold is breached."] clientTradeID : Option < ClientID > , # [doc = " The price threshold specified for the TakeProfit Order. The associated Trade will be closed by a market price that is equal to or better than this threshold."] price : PriceValue , # [doc = " The time-in-force requested for the TakeProfit Order. Restricted to “GTC”, “GFD” and “GTD” for TakeProfit Orders."] # [serde (default = "GTC")] timeInForce : TimeInForce , # [doc = " The date/time when the TakeProfit Order will be cancelled if its timeInForce is “GTD”."] gtdTime : Option < DateTime > , # [doc = " Specification of which price component should be used when determining if an Order should be triggered and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always filled using their default price component. This feature is only provided through the REST API. Clients who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s proprietary or partner trading platforms, their transaction history or their account statements. OANDA platforms always assume that an Order’s trigger condition is set to the default value when indicating the distance from an Order’s trigger price, and will always provide the default trigger condition when creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss Order. In this case the TriggerCondition value must either be “DEFAULT”, or the “natural” trigger side “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT” and “BID”, and for short trades “DEFAULT” and “ASK” are valid."] # [serde (default = "DEFAULT")] triggerCondition : OrderTriggerCondition , # [doc = " ID of the Transaction that filled this Order (only provided when the Order’s state is FILLED)"] fillingTransactionID : Option < TransactionID > , # [doc = " Date/time when the Order was filled (only provided when the Order’s state is FILLED)"] filledTime : Option < DateTime > , # [doc = " Trade ID of Trade opened when the Order was filled (only provided when the Order’s state is FILLED and a Trade was opened as a result of the fill)"] tradeOpenedID : Option < TradeID > , # [doc = " Trade ID of Trade reduced when the Order was filled (only provided when the Order’s state is FILLED and a Trade was reduced as a result of the fill)"] tradeReducedID : Option < TradeID > , # [doc = " Trade IDs of Trades closed when the Order was filled (only provided when the Order’s state is FILLED and one or more Trades were closed as a result of the fill)"] tradeClosedIDs : Vec < TradeID > , # [doc = " ID of the Transaction that cancelled the Order (only provided when the Order’s state is CANCELLED)"] cancellingTransactionID : Option < TransactionID > , # [doc = " Date/time when the Order was cancelled (only provided when the state of the Order is CANCELLED)"] cancelledTime : Option < DateTime > , # [doc = " The ID of the Order that was replaced by this Order (only provided if this Order was created as part of a cancel/replace)."] replacesOrderID : Option < OrderID > , # [doc = " The ID of the Order that replaced this Order (only provided if this Order was cancelled as part of a cancel/replace)."] replacedByOrderID : Option < OrderID > , }