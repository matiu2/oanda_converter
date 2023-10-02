use serde :: { Serialize , Deserialize } ; _blank_ ! () ; # [derive (Serialize , Deserialize)] struct OrderClientExtensionsModifyTransaction { # [doc = " The Transaction’s Identifier."] id : Option < TransactionID > , # [doc = " The date/time when the Transaction was created."] time : Option < DateTime > , # [doc = " The ID of the user that initiated the creation of the Transaction."] userID : Option < integer > , # [doc = " The ID of the Account the Transaction was created for."] accountID : Option < AccountID > , # [doc = " The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the Account simultaneously."] batchID : Option < TransactionID > , # [doc = " The Request ID of the request which generated the transaction."] requestID : Option < RequestID > , # [doc = " The Type of the Transaction. Always set to “ORDER_CLIENT_EXTENSIONS_MODIFY” for a OrderClientExtensionsModifyTransaction."] # [serde (default = "ORDER_CLIENT_EXTENSIONS_MODIFY")] type : TransactionType , # [doc = " The ID of the Order who’s client extensions are to be modified."] orderID : Option < OrderID > , # [doc = " The original Client ID of the Order who’s client extensions are to be modified."] clientOrderID : Option < ClientID > , # [doc = " The new Client Extensions for the Order."] clientExtensionsModify : Option < ClientExtensions > , # [doc = " The new Client Extensions for the Order’s Trade on fill."] tradeClientExtensionsModify : Option < ClientExtensions > , }