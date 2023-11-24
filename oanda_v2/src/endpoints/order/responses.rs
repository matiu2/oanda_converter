/// The Order was created as specified
#[derive(Serialize, Deserialize)]
struct Orders201 {
    /// The Transaction that created the Order specified by the
    /// request.
    order_create_transaction: Option<Transaction>,
    /// The Transaction that filled the newly created Order. Only
    /// provided when the Order was immediately filled.
    order_fill_transaction: Option<OrderFillTransaction>,
    /// The Transaction that cancelled the newly created Order. Only
    /// provided when the Order was immediately cancelled.
    order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction that reissues the Order. Only provided when
    /// the Order is configured to be reissued for its remaining
    /// units after a partial fill and the reissue was successful.
    order_reissue_transaction: Option<Transaction>,
    /// The Transaction that rejects the reissue of the Order. Only
    /// provided when the Order is configured to be reissued for
    /// its remaining units after a partial fill and the reissue
    /// was rejected.
    order_reissue_reject_transaction: Option<Transaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
#[derive(Serialize, Deserialize)]
struct Orders400 {
    /// The Transaction that rejected the creation of the Order
    /// as requested
    order_reject_transaction: Option<Transaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
#[derive(Serialize, Deserialize)]
struct Orders404 {
    /// The Transaction that rejected the creation of the Order as
    /// requested. Only present if the Account exists.
    order_reject_transaction: Option<Transaction>,
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
/// The list of Orders requested
#[derive(Serialize, Deserialize)]
struct Orders200 {
    /// The list of Order detail objects
    orders: Vec<Order>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
/// List of pending Orders for the Account
#[derive(Serialize, Deserialize)]
struct PendingOrders200 {
    /// The list of pending Order details
    orders: Vec<Order>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
/// The details of the Order requested
#[derive(Serialize, Deserialize)]
struct Get200 {
    /// The details of the Order requested
    order: Option<Order>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
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
/// The Order was cancelled as specified
#[derive(Serialize, Deserialize)]
struct Cancel200 {
    /// The Transaction that cancelled the Order
    order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
#[derive(Serialize, Deserialize)]
struct Cancel404 {
    /// The Transaction that rejected the cancellation of the Order.
    /// Only present if the Account exists.
    order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
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
/// The Order’s Client Extensions were successfully modified
#[derive(Serialize, Deserialize)]
struct ClientExtensions200 {
    /// The Transaction that modified the Client Extensions for
    /// the Order
    order_client_extensions_modify_transaction: Option<
        OrderClientExtensionsModifyTransaction,
    >,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
}
#[derive(Serialize, Deserialize)]
struct ClientExtensions400 {
    /// The Transaction that rejected the modification of the Client
    /// Extensions for the Order
    order_client_extensions_modify_reject_transaction: Option<
        OrderClientExtensionsModifyRejectTransaction,
    >,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
#[derive(Serialize, Deserialize)]
struct ClientExtensions404 {
    /// The Transaction that rejected the modification of the
    /// Client Extensions for the Order. Only present if the Account
    /// exists.
    order_client_extensions_modify_reject_transaction: Option<
        OrderClientExtensionsModifyRejectTransaction,
    >,
    /// The ID of the most recent Transaction created for the
    /// Account. Only present if the Account exists.
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request. Only present if the Account exists.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
