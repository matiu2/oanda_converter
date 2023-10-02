use serde :: { Serialize , Deserialize } ; _blank_ ! () ; # [derive (Serialize , Deserialize)] struct TransferFundsRejectTransaction { # [doc = " The Transaction’s Identifier."] id : Option < TransactionID > , # [doc = " The date/time when the Transaction was created."] time : Option < DateTime > , # [doc = " The ID of the user that initiated the creation of the Transaction."] userID : Option < integer > , # [doc = " The ID of the Account the Transaction was created for."] accountID : Option < AccountID > , # [doc = " The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the Account simultaneously."] batchID : Option < TransactionID > , # [doc = " The Request ID of the request which generated the transaction."] requestID : Option < RequestID > , # [doc = " The Type of the Transaction. Always set to “TRANSFER_FUNDS_REJECT” in a TransferFundsRejectTransaction."] # [serde (default = "TRANSFER_FUNDS_REJECT")] type : TransactionType , # [doc = " The amount to deposit/withdraw from the Account in the Account’s home currency. A positive value indicates a deposit, a negative value indicates a withdrawal."] amount : Option < AccountUnits > , # [doc = " The reason that an Account is being funded."] fundingReason : Option < FundingReason > , # [doc = " An optional comment that may be attached to a fund transfer for audit purposes"] comment : Option < string > , # [doc = " The reason that the Reject Transaction was created"] rejectReason : Option < TransactionRejectReason > , }