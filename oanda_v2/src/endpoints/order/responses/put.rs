/// The Order was successfully cancelled and replaced
#[derive(Serialize, Deserialize)]
struct Put201 {
    /// The Transaction that cancelled the Order to be replaced.
    order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction that created the replacing Order as
    /// requested.
    order_create_transaction: Option<Transaction>,
    /// The Transaction that filled the replacing Order. This is
    /// only provided when the replacing Order was immediately
    /// filled.
    order_fill_transaction: Option<OrderFillTransaction>,
    /// The Transaction that reissues the replacing Order. Only
    /// provided when the replacing Order was partially filled
    /// immediately and is configured to be reissued for its
    /// remaining units.
    order_reissue_transaction: Option<Transaction>,
    /// The Transaction that rejects the reissue of the Order.
    /// Only provided when the replacing Order was partially filled
    /// immediately and was configured to be reissued, however the
    /// reissue was rejected.
    order_reissue_reject_transaction: Option<Transaction>,
    /// The Transaction that cancelled the replacing Order. Only
    /// provided when the replacing Order was immediately cancelled.
    replacing_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
#[derive(Serialize, Deserialize)]
struct Put400 {
    /// The Transaction that rejected the creation of the replacing
    /// Order
    order_reject_transaction: Option<Transaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
#[derive(Serialize, Deserialize)]
struct Put404 {
    /// The Transaction that rejected the cancellation of the Order
    /// to be replaced. Only present if the Account exists.
    order_cancel_reject_transaction: Option<Transaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request. Only present if the Account exists.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account. Only present if the Account exists.
    last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
#[derive(Debug)]
pub enum Error {
    E400(Put400),
    E404(Put404),
}
