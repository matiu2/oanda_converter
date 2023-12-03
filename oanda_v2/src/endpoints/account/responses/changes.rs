/// The Account state and changes are provided.
#[derive(Serialize, Deserialize)]
struct Changes200 {
    /// The changes to the Account’s Orders, Trades and Positions
    /// since the specified Transaction ID. Only provided if the
    /// sinceTransactionID is supplied to the poll request.
    changes: Option<AccountChanges>,
    /// The Account’s current price-dependent state.
    state: Option<AccountChangesState>,
    /// The ID of the last Transaction created for the Account.
    /// This Transaction ID should be used for future poll requests,
    /// as the client has already observed all changes up to and
    /// including it.
    last_transaction_id: Option<TransactionID>,
}
